// rv64i_cpu.v — RISC-V 64IM + Zicsr + Zifencei + C Extension 單週期處理器

`define ALU_ADD   4'b0000
`define ALU_SUB   4'b0001
`define ALU_SLL   4'b0010
`define ALU_SLT   4'b0011
`define ALU_SLTU  4'b0100
`define ALU_XOR   4'b0101
`define ALU_SRL   4'b0110
`define ALU_SRA   4'b0111
`define ALU_OR    4'b1000
`define ALU_AND   4'b1001
`define ALU_PASS  4'b1010
`define ALU_MUL   4'b1011
`define ALU_MULH  4'b1100
`define ALU_MULHU 4'b1101
`define ALU_DIV   4'b1110
`define ALU_DIVU  4'b1111

module rv64i_cpu (
    input  wire       clk,
    input  wire       rst_n,
    output wire       dbg_ecall
);

    // ====== Program Counter ======
    reg [63:0] pc;
    wire [63:0] pc_inc;
    wire [63:0] pc_next;

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n)
            pc <= 64'b0;
        else
            pc <= pc_next;
    end

    // ====== Instruction Memory (32-bit words, 32KB) ======
    reg [31:0] imem [0:8191];

    wire [12:0] word_addr = pc[14:2];
    wire        half_sel  = pc[1];

    wire [15:0] lower_half = imem[word_addr][15:0];
    wire [15:0] upper_half = imem[word_addr][31:16];

    wire [15:0] fetch_half = half_sel ? upper_half : lower_half;
    wire        is_compressed = (fetch_half[1:0] != 2'b11);

    // Full 32-bit instruction: either one word or cross-word combination
    wire [31:0] instr;
    assign instr = is_compressed ? {16'b0, fetch_half} :
                   half_sel ? {imem[word_addr+1][15:0], upper_half} :
                   imem[word_addr];

    integer init_i;
    initial begin
        for (init_i = 0; init_i < 8192; init_i = init_i + 1)
            imem[init_i] = 32'h00000013;
        $readmemh("program.hex", imem);
    end

    // ====== Compressed (16-bit) / Uncompressed (32-bit) Decode ======
    wire [6:0] opcode  = instr[6:0];
    wire [4:0] rd      = instr[11:7];
    wire [2:0] funct3  = instr[14:12];
    wire [4:0] rs1     = instr[19:15];
    wire [4:0] rs2     = instr[24:20];
    wire [6:0] funct7  = instr[31:25];

    // RVC decoded fields
    wire [1:0] rvc_q      = fetch_half[1:0];
    wire [2:0] rvc_funct3 = fetch_half[15:13];
    wire       rvc_bit12  = fetch_half[12];
    wire [4:0] rvc_rd     = fetch_half[11:7];
    wire [4:0] rvc_rs2    = fetch_half[6:2];
    wire [2:0] rvc_rdq    = fetch_half[4:2];
    wire [2:0] rvc_rs1q   = fetch_half[9:7];
    wire [2:0] rvc_rs2q   = fetch_half[4:2];

    wire [4:0] rvc_rd5  = is_compressed ?
                             (rvc_q == 2'b00) ? {2'b01, rvc_rdq} :
                             (rvc_q == 2'b01 && rvc_funct3 == 3'b100 && fetch_half[11:10] == 2'b11) ? {2'b01, rvc_rs1q} :
                             rvc_rd :
                             rd;
    wire [4:0] rvc_rs15 = is_compressed ?
                             (rvc_q == 2'b00 && rvc_funct3 == 3'b000) ? 5'b00010 : // C.ADDI4SPN uses sp (x2)
                             (rvc_q == 2'b00) ? {2'b01, rvc_rs1q} :
                             (rvc_q == 2'b01 && rvc_funct3 == 3'b100 && fetch_half[11:10] == 2'b11) ? {2'b01, rvc_rs1q} :
                             (rvc_q == 2'b01 && (rvc_funct3 == 3'b110 || rvc_funct3 == 3'b111)) ? {2'b01, rvc_rs1q} :
                             (rvc_q == 2'b10 && rvc_funct3 == 3'b100 && rvc_bit12 == 0 && rvc_rs2 != 0) ? 5'b0 :
                             rvc_rd :
                             rs1;
    wire [4:0] rvc_rs25 = is_compressed ?
                             (rvc_q == 2'b00) ? {2'b01, rvc_rs2q} :
                             (rvc_q == 2'b01 && rvc_funct3 == 3'b100 && fetch_half[11:10] == 2'b11) ? {2'b01, fetch_half[4:2]} :
                             rvc_rs2 :
                             rs2;

    // ====== RVC Immediate Generator ======
    wire [5:0] rvc_imm_i6 = {rvc_bit12, fetch_half[6:2]};
    wire [63:0] rvc_imm = {{58{rvc_imm_i6[5]}}, rvc_imm_i6};

    wire [11:0] rvc_imm_j11 = {fetch_half[12], fetch_half[8], fetch_half[10:9],
                                fetch_half[6], fetch_half[7], fetch_half[2],
                                fetch_half[11], fetch_half[5:3], 1'b0};
    wire [63:0] rvc_imm_j = {{52{rvc_imm_j11[11]}}, rvc_imm_j11};

    wire [8:0] rvc_imm_b9 = {fetch_half[12], fetch_half[6:5],
                              fetch_half[2], fetch_half[11:10],
                              fetch_half[4:3], 1'b0};
    wire [63:0] rvc_imm_b = {{55{rvc_imm_b9[8]}}, rvc_imm_b9};

    wire [9:0] rvc_nzuimm10 = {fetch_half[10:7], fetch_half[12:11], fetch_half[5], fetch_half[6], 2'b00};
    wire [63:0] rvc_imm_addi4spn = {54'b0, rvc_nzuimm10};

    wire [7:0] rvc_uimm_ld = {fetch_half[11:10], fetch_half[7:6], fetch_half[12], 3'b000};
    wire [63:0] rvc_imm_ld = {56'b0, rvc_uimm_ld};

    wire [63:0] rvc_imm_sd = rvc_imm_ld;

    wire [6:0] rvc_uimm_lw = {fetch_half[11:10], fetch_half[12], fetch_half[6], fetch_half[7], 2'b00};
    wire [63:0] rvc_imm_lw = {57'b0, rvc_uimm_lw};

    wire [63:0] rvc_imm_sw = rvc_imm_lw;

    wire [9:0] rvc_imm_addi16sp_val = {fetch_half[12], fetch_half[4:3],
                                        fetch_half[5], fetch_half[2],
                                        fetch_half[6], 4'b0000};
    wire [63:0] rvc_imm_addi16sp = {{54{rvc_imm_addi16sp_val[9]}}, rvc_imm_addi16sp_val};

    wire [63:0] rvc_imm_lui = {{43{rvc_imm_i6[5]}}, rvc_imm_i6, 12'b0};

    wire [7:0] rvc_uimm_ldsp = {fetch_half[11:10], fetch_half[8:7], fetch_half[12], fetch_half[6], 3'b000};
    wire [63:0] rvc_imm_ldsp = {56'b0, rvc_uimm_ldsp};

    wire [6:0] rvc_uimm_lwsp = {fetch_half[11:10], fetch_half[8:7], fetch_half[12], fetch_half[6], 2'b00};
    wire [63:0] rvc_imm_lwsp = {57'b0, rvc_uimm_lwsp};

    wire [7:0] rvc_uimm_sdsp = {fetch_half[11:10], fetch_half[9:7], fetch_half[12], fetch_half[6], 3'b000};
    wire [63:0] rvc_imm_sdsp = {56'b0, rvc_uimm_sdsp};

    wire [6:0] rvc_uimm_swsp = {fetch_half[11:10], fetch_half[9:7], fetch_half[12], fetch_half[6], 2'b00};
    wire [63:0] rvc_imm_swsp = {57'b0, rvc_uimm_swsp};

    // ====== Register File ======
    reg [63:0] rf [0:31];
    wire [63:0] rf_rdata1 = (rvc_rs15 != 0) ? rf[rvc_rs15] : 64'b0;
    wire [63:0] rf_rdata2 = (rvc_rs25 != 0) ? rf[rvc_rs25] : 64'b0;

    // ====== Data Memory (byte-addressable, 8KB) ======
    reg [7:0] dmem [0:8191];
    wire [12:0] dmem_addr = alu_result[12:0];

    // ====== Immediate Generator (32-bit instructions) ======
    wire [63:0] imm_i = {{52{instr[31]}}, instr[31:20]};
    wire [63:0] imm_s = {{52{instr[31]}}, instr[31:25], instr[11:7]};
    wire [63:0] imm_b = {{51{instr[31]}}, instr[31], instr[7], instr[30:25], instr[11:8], 1'b0};
    wire [63:0] imm_u = {{32{instr[31]}}, instr[31:12], 12'b0};
    wire [63:0] imm_j = {{43{instr[31]}}, instr[31], instr[19:12], instr[20], instr[30:21], 1'b0};
    wire [63:0] imm_z = {59'b0, rs1};

    // ====== CSR File ======
    reg [63:0] mcycle;
    reg [63:0] minstret;
    reg [63:0] mstatus;
    reg [63:0] mtvec;
    reg [63:0] mepc;
    reg [63:0] mcause;

    reg [63:0]  csr_rdata;
    reg         csr_write;
    reg [63:0]  csr_wdata;

    always @(*) begin
        case (instr[31:20])
            12'h300: csr_rdata = mstatus;
            12'h305: csr_rdata = mtvec;
            12'h341: csr_rdata = mepc;
            12'h342: csr_rdata = mcause;
            12'hB00: csr_rdata = mcycle;
            12'hB02: csr_rdata = minstret;
            default: csr_rdata = 64'b0;
        endcase
    end

    // ====== ECALL / MRET Detection ======
    wire [11:0] funct12 = instr[31:20];
    wire is_ecall = !is_compressed && opcode == 7'b1110011 && funct3 == 3'b000 && funct12 == 12'h000;
    wire is_mret  = !is_compressed && opcode == 7'b1110011 && funct3 == 3'b000 && funct12 == 12'h302;

    assign dbg_ecall = is_ecall;

    // ====== PC Increment ======
    assign pc_inc = is_compressed ? 64'd2 : 64'd4;

    // ====== Control Unit ======
    reg        reg_write;
    reg        mem_write;
    reg        branch;
    reg        jump;
    reg        jalr;
    reg [1:0]  reg_src;
    reg        alu_src_a;
    reg [1:0]  alu_src_b;

    reg        csr_op;
    reg        csr_imm;

    always @(*) begin
        reg_write = 1'b0; mem_write = 1'b0;
        branch = 1'b0; jump = 1'b0; jalr = 1'b0;
        reg_src = 2'b00; alu_src_a = 1'b0; alu_src_b = 2'b00;
        csr_op = 1'b0; csr_write = 1'b0; csr_imm = 1'b0;

        if (is_compressed) begin
            if (rvc_q == 2'b01) begin
                case (rvc_funct3)
                    3'b000: begin // C.ADDI
                        reg_write = (rvc_rd != 0 && rvc_imm_i6 != 0);
                        alu_src_a = 1'b0; alu_src_b = 2'b01;
                        reg_src = 2'b00;
                    end
                    3'b001: begin // C.ADDIW
                        reg_write = (rvc_rd != 0);
                        alu_src_a = 1'b0; alu_src_b = 2'b01;
                        reg_src = 2'b00;
                    end
                    3'b010: begin // C.LI
                        reg_write = (rvc_rd != 0);
                        reg_src = 2'b11;
                    end
                    3'b011: begin // C.ADDI16SP or C.LUI
                        if (rvc_rd == 5'b00010) begin
                            reg_write = 1'b1;
                            alu_src_a = 1'b0; alu_src_b = 2'b01;
                            reg_src = 2'b00;
                        end else if (rvc_rd != 0) begin
                            reg_write = 1'b1;
                            reg_src = 2'b11;
                        end
                    end
                    3'b100: begin
                        if (rvc_bit12 == 0 && fetch_half[11:10] == 2'b00) begin // C.SRLI
                            reg_write = 1'b1;
                            alu_src_a = 1'b0; alu_src_b = 2'b01;
                            reg_src = 2'b00;
                        end else if (rvc_bit12 == 1 && fetch_half[11:10] == 2'b00) begin // C.SRAI
                            reg_write = 1'b1;
                            alu_src_a = 1'b0; alu_src_b = 2'b01;
                            reg_src = 2'b00;
                        end else if (fetch_half[11:10] == 2'b10) begin // C.ANDI
                            reg_write = 1'b1;
                            alu_src_a = 1'b0; alu_src_b = 2'b01;
                            reg_src = 2'b00;
                        end else if (fetch_half[11:10] == 2'b11 && fetch_half[6:5] == 2'b00) begin // C.SUB
                            reg_write = (rvc_rd != 0);
                            alu_src_a = 1'b0; alu_src_b = 2'b00;
                            reg_src = 2'b00;
                        end else if (fetch_half[11:10] == 2'b11 && fetch_half[6:5] == 2'b10) begin // C.XOR (bit12=0) / C.AND (bit12=1)
                            reg_write = (rvc_rd != 0);
                            alu_src_a = 1'b0; alu_src_b = 2'b00;
                            reg_src = 2'b00;
                        end
                    end
                    3'b101: begin // C.J
                        jump = 1'b1;
                    end
                    3'b110: begin // C.BEQZ
                        branch = 1'b1;
                    end
                    3'b111: begin // C.BNEZ
                        branch = 1'b1;
                    end
                endcase
            end else if (rvc_q == 2'b10) begin
                case (rvc_funct3)
                    3'b000: begin // C.SLLI
                        reg_write = (rvc_rd != 0);
                        alu_src_a = 1'b0; alu_src_b = 2'b01;
                        reg_src = 2'b00;
                    end
                    3'b001: begin // C.LWSP
                        reg_write = (rvc_rd != 0);
                        alu_src_a = 1'b0; alu_src_b = 2'b01;
                        reg_src = 2'b01;
                    end
                    3'b010: begin // C.LDSP
                        reg_write = (rvc_rd != 0);
                        alu_src_a = 1'b0; alu_src_b = 2'b01;
                        reg_src = 2'b01;
                    end
                    3'b100: begin // C.MV / C.ADD / C.JR / C.JALR / C.EBREAK
                        if (rvc_rs2 != 0 && rvc_bit12 == 0) begin // C.MV
                            reg_write = (rvc_rd != 0);
                            reg_src = 2'b11;
                        end else if (rvc_rs2 != 0 && rvc_bit12 == 1) begin // C.ADD
                            reg_write = (rvc_rd != 0);
                            alu_src_a = 1'b0; alu_src_b = 2'b00;
                            reg_src = 2'b00;
                        end else if (rvc_rs2 == 0 && rvc_bit12 == 0 && rvc_rd != 0) begin // C.JR
                            jump = 1'b1; jalr = 1'b1;
                        end else if (rvc_rs2 == 0 && rvc_bit12 == 1 && rvc_rd != 0) begin // C.JALR
                            jump = 1'b1; jalr = 1'b1;
                            reg_write = 1'b1;
                            reg_src = 2'b10;
                        end
                    end
                    3'b110: begin // C.SWSP
                        mem_write = 1'b1;
                        alu_src_a = 1'b0; alu_src_b = 2'b01;
                    end
                    3'b111: begin // C.SDSP
                        mem_write = 1'b1;
                        alu_src_a = 1'b0; alu_src_b = 2'b01;
                    end
                endcase
            end else if (rvc_q == 2'b00) begin
                case (rvc_funct3)
                    3'b000: begin // C.ADDI4SPN
                        reg_write = 1'b1;
                        alu_src_a = 1'b0; alu_src_b = 2'b01;
                        reg_src = 2'b00;
                    end
                    3'b001: begin // C.LW
                        reg_write = 1'b1;
                        alu_src_a = 1'b0; alu_src_b = 2'b01;
                        reg_src = 2'b01;
                    end
                    3'b011: begin // C.LD
                        reg_write = 1'b1;
                        alu_src_a = 1'b0; alu_src_b = 2'b01;
                        reg_src = 2'b01;
                    end
                    3'b010: begin // C.SW
                        mem_write = 1'b1;
                        alu_src_a = 1'b0; alu_src_b = 2'b01;
                    end
                    3'b110: begin // C.SD
                        mem_write = 1'b1;
                        alu_src_a = 1'b0; alu_src_b = 2'b01;
                    end
                endcase
            end
        end else begin
            case (opcode)
                7'b0110011: begin
                    reg_write = 1'b1; alu_src_a = 1'b0; alu_src_b = 2'b00;
                    reg_src = 2'b00;
                end
                7'b0010011: begin
                    reg_write = 1'b1; alu_src_a = 1'b0; alu_src_b = 2'b01;
                    reg_src = 2'b00;
                end
                7'b0000011: begin
                    reg_write = 1'b1;
                    alu_src_a = 1'b0; alu_src_b = 2'b01;
                    reg_src = 2'b01;
                end
                7'b0100011: begin
                    mem_write = 1'b1;
                    alu_src_a = 1'b0; alu_src_b = 2'b01;
                end
                7'b1100011: begin
                    branch = 1'b1;
                    alu_src_a = 1'b0; alu_src_b = 2'b00;
                end
                7'b0110111: begin
                    reg_write = 1'b1;
                    reg_src = 2'b11;
                end
                7'b0010111: begin
                    reg_write = 1'b1;
                    alu_src_a = 1'b1; alu_src_b = 2'b10;
                    reg_src = 2'b00;
                end
                7'b1101111: begin
                    reg_write = 1'b1; jump = 1'b1;
                    reg_src = 2'b10;
                end
                7'b1100111: begin
                    reg_write = 1'b1; jump = 1'b1; jalr = 1'b1;
                    alu_src_a = 1'b0; alu_src_b = 2'b01;
                    reg_src = 2'b10;
                end
                7'b1110011: begin
                    if (!is_ecall && !is_mret) begin
                        csr_op = 1'b1;
                        reg_write = 1'b1;
                        if (funct3 == 3'b001 || funct3 == 3'b101) begin
                            csr_write = 1'b1;
                        end else if (funct3 == 3'b010 || funct3 == 3'b110) begin
                            if (rs1 != 0) csr_write = 1'b1;
                        end else if (funct3 == 3'b011 || funct3 == 3'b111) begin
                            if (rs1 != 0) csr_write = 1'b1;
                        end
                        if (funct3[2]) csr_imm = 1'b1;
                    end
                end
            endcase
        end
    end

    // ====== ALU Source B ======
    reg [63:0]  alu_b;
    reg [3:0]   alu_ctrl;
    wire [63:0] alu_a = alu_src_a ? pc : rf_rdata1;

    wire [63:0] rvc_alu_imm;
    assign rvc_alu_imm = (rvc_q == 2'b00) ?
                            (rvc_funct3 == 3'b000 ? rvc_imm_addi4spn :
                             rvc_funct3 == 3'b011 ? rvc_imm_ld :
                             rvc_funct3 == 3'b001 ? rvc_imm_lw :
                             rvc_funct3 == 3'b110 ? rvc_imm_sd :
                             rvc_funct3 == 3'b010 ? rvc_imm_sw :
                             rvc_imm) :
                         (rvc_q == 2'b01) ?
                            (rvc_funct3 == 3'b011 && rvc_rd == 5'b00010 ? rvc_imm_addi16sp :
                             rvc_funct3 == 3'b100 && fetch_half[11:10] == 2'b10 ? {{58{fetch_half[12]}}, fetch_half[12], fetch_half[6:2]} :
                             rvc_funct3 == 3'b100 && (fetch_half[11:10] == 2'b00) ? {59'b0, fetch_half[12], fetch_half[6:2]} :
                             rvc_imm) :
                         (rvc_q == 2'b10) ?
                            (rvc_funct3 == 3'b000 ? {59'b0, rvc_rd[4:0]} :
                             rvc_funct3 == 3'b001 ? rvc_imm_lwsp :
                             rvc_funct3 == 3'b010 ? rvc_imm_ldsp :
                             rvc_funct3 == 3'b110 ? rvc_imm_swsp :
                             rvc_funct3 == 3'b111 ? rvc_imm_sdsp :
                             rvc_imm) :
                         64'b0;

    always @(*) begin
        if (is_compressed) begin
            case (alu_src_b)
                2'b00: alu_b = rf_rdata2;
                default: alu_b = rvc_alu_imm;
            endcase
        end else begin
            case (alu_src_b)
                2'b00: alu_b = rf_rdata2;
                2'b01: alu_b = (opcode == 7'b0100011) ? imm_s : imm_i;
                2'b10: alu_b = imm_u;
                2'b11: alu_b = 64'd4;
                default: alu_b = rf_rdata2;
            endcase
        end
    end

    // ====== ALU Control ======
    always @(*) begin
        if (is_compressed) begin
            if (rvc_q == 2'b01 && rvc_funct3 == 3'b000) begin
                alu_ctrl = `ALU_ADD;
            end else if (rvc_q == 2'b01 && rvc_funct3 == 3'b001) begin
                alu_ctrl = `ALU_ADD;
            end else if (rvc_q == 2'b01 && rvc_funct3 == 3'b011 && rvc_rd == 5'b00010) begin
                alu_ctrl = `ALU_ADD;
            end else if (rvc_q == 2'b01 && rvc_funct3 == 3'b100) begin
                if (fetch_half[11:10] == 2'b00 && rvc_bit12 == 0) begin
                    alu_ctrl = `ALU_SRL;
                end else if (fetch_half[11:10] == 2'b00 && rvc_bit12 == 1) begin
                    alu_ctrl = `ALU_SRA;
                end else if (fetch_half[11:10] == 2'b10) begin
                    alu_ctrl = `ALU_AND;
                end else if (fetch_half[11:10] == 2'b11 && fetch_half[6:5] == 2'b00) begin
                    alu_ctrl = rvc_bit12 ? `ALU_OR : `ALU_SUB;
                end else if (fetch_half[11:10] == 2'b11 && fetch_half[6:5] == 2'b10) begin
                    alu_ctrl = rvc_bit12 ? `ALU_AND : `ALU_XOR;
                end else begin
                    alu_ctrl = `ALU_ADD;
                end
            end else if (rvc_q == 2'b00 && rvc_funct3 == 3'b000) begin
                alu_ctrl = `ALU_ADD;
            end else if (rvc_q == 2'b10 && rvc_funct3 == 3'b000) begin
                alu_ctrl = `ALU_SLL;
            end else if (rvc_q == 2'b10 && rvc_funct3 == 3'b100 && rvc_rs2 != 0 && rvc_bit12 == 1) begin
                alu_ctrl = `ALU_ADD;
            end else begin
                alu_ctrl = `ALU_ADD;
            end
        end else if (opcode == 7'b0110011) begin
            case ({funct7, funct3})
                {7'b0000000, 3'b000}: alu_ctrl = `ALU_ADD;
                {7'b0100000, 3'b000}: alu_ctrl = `ALU_SUB;
                {7'b0000000, 3'b001}: alu_ctrl = `ALU_SLL;
                {7'b0000000, 3'b010}: alu_ctrl = `ALU_SLT;
                {7'b0000000, 3'b011}: alu_ctrl = `ALU_SLTU;
                {7'b0000000, 3'b100}: alu_ctrl = `ALU_XOR;
                {7'b0000000, 3'b101}: alu_ctrl = `ALU_SRL;
                {7'b0100000, 3'b101}: alu_ctrl = `ALU_SRA;
                {7'b0000000, 3'b110}: alu_ctrl = `ALU_OR;
                {7'b0000000, 3'b111}: alu_ctrl = `ALU_AND;
                {7'b0000001, 3'b000}: alu_ctrl = `ALU_MUL;
                {7'b0000001, 3'b001}: alu_ctrl = `ALU_MULH;
                {7'b0000001, 3'b010}: alu_ctrl = `ALU_MULH;
                {7'b0000001, 3'b011}: alu_ctrl = `ALU_MULHU;
                {7'b0000001, 3'b100}: alu_ctrl = `ALU_DIV;
                {7'b0000001, 3'b101}: alu_ctrl = `ALU_DIVU;
                {7'b0000001, 3'b110}: alu_ctrl = `ALU_DIV;
                {7'b0000001, 3'b111}: alu_ctrl = `ALU_DIVU;
                default: alu_ctrl = `ALU_ADD;
            endcase
        end else if (opcode == 7'b0010011) begin
            case ({funct7, funct3})
                {7'b0000000, 3'b000}: alu_ctrl = `ALU_ADD;
                {7'b0100000, 3'b000}: alu_ctrl = `ALU_SUB;
                {7'b0000000, 3'b001}: alu_ctrl = `ALU_SLL;
                {7'b0000000, 3'b010}: alu_ctrl = `ALU_SLT;
                {7'b0000000, 3'b011}: alu_ctrl = `ALU_SLTU;
                {7'b0000000, 3'b100}: alu_ctrl = `ALU_XOR;
                {7'b0000000, 3'b101}: alu_ctrl = `ALU_SRL;
                {7'b0100000, 3'b101}: alu_ctrl = `ALU_SRA;
                {7'b0000000, 3'b110}: alu_ctrl = `ALU_OR;
                {7'b0000000, 3'b111}: alu_ctrl = `ALU_AND;
                default: alu_ctrl = `ALU_ADD;
            endcase
        end else if (opcode == 7'b0110111) begin
            alu_ctrl = `ALU_PASS;
        end else begin
            alu_ctrl = `ALU_ADD;
        end
    end

    // ====== CSR Write Data ======
    wire [63:0] csr_rs1_val = csr_imm ? imm_z : rf_rdata1;

    always @(*) begin
        if (!csr_op) begin
            csr_wdata = 64'b0;
        end else begin
            case (funct3)
                3'b001: csr_wdata = csr_rs1_val;
                3'b010: csr_wdata = csr_rdata | csr_rs1_val;
                3'b011: csr_wdata = csr_rdata & ~csr_rs1_val;
                3'b101: csr_wdata = csr_rs1_val;
                3'b110: csr_wdata = csr_rdata | csr_rs1_val;
                3'b111: csr_wdata = csr_rdata & ~csr_rs1_val;
                default: csr_wdata = csr_rs1_val;
            endcase
        end
    end

    // ====== CSR Write (posedge) ======
    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            mcycle   <= 64'b0;
            minstret <= 64'b0;
            mstatus  <= 64'b0;
            mtvec    <= 64'b0;
            mepc     <= 64'b0;
            mcause   <= 64'b0;
        end else begin
            mcycle <= mcycle + 1;
            minstret <= minstret + 1;
            if (is_ecall) begin
                mepc   <= pc;
                mcause <= 64'd11;
            end
            if (csr_write && !is_ecall && !is_mret) begin
                case (instr[31:20])
                    12'h300: mstatus <= csr_wdata;
                    12'h305: mtvec   <= csr_wdata;
                    12'h341: mepc    <= csr_wdata;
                    12'h342: mcause  <= csr_wdata;
                    12'hB00: mcycle  <= csr_wdata;
                    12'hB02: minstret <= csr_wdata;
                endcase
            end
        end
    end

    // ====== ALU Datapath ======
    reg [63:0] alu_result;
    wire [127:0] mul_full_s  = $signed(alu_a) * $signed(alu_b);
    wire [127:0] mul_full_u  = alu_a * alu_b;
    wire [127:0] mul_full_su = $signed(alu_a) * $signed({1'b0, alu_b});

    always @(*) begin
        case (alu_ctrl)
            `ALU_ADD:  alu_result = alu_a + alu_b;
            `ALU_SUB:  alu_result = alu_a - alu_b;
            `ALU_SLL:  alu_result = alu_a << alu_b[5:0];
            `ALU_SLT:  alu_result = ($signed(alu_a) < $signed(alu_b)) ? 64'd1 : 64'd0;
            `ALU_SLTU: alu_result = (alu_a < alu_b) ? 64'd1 : 64'd0;
            `ALU_XOR:  alu_result = alu_a ^ alu_b;
            `ALU_SRL:  alu_result = alu_a >> alu_b[5:0];
            `ALU_SRA:  alu_result = $signed(alu_a) >>> alu_b[5:0];
            `ALU_OR:   alu_result = alu_a | alu_b;
            `ALU_AND:  alu_result = alu_a & alu_b;
            `ALU_PASS: alu_result = alu_b;
            `ALU_MUL: begin
                alu_result = mul_full_s[63:0];
            end
            `ALU_MULH: begin
                if (funct3 == 3'b010)
                    alu_result = mul_full_su[127:64];
                else
                    alu_result = mul_full_s[127:64];
            end
            `ALU_MULHU: alu_result = mul_full_u[127:64];
            `ALU_DIV: begin
                if (alu_b == 0)
                    alu_result = ~64'b0;
                else if (funct3 == 3'b110)
                    alu_result = $signed(alu_a) % $signed(alu_b);
                else if (funct3 == 3'b100 && $signed(alu_a) == (1 << 63) && $signed(alu_b) == -1)
                    alu_result = (1 << 63);
                else if (funct3 == 3'b100)
                    alu_result = $signed(alu_a) / $signed(alu_b);
                else
                    alu_result = alu_a;
            end
            `ALU_DIVU: begin
                if (alu_b == 0)
                    alu_result = ~64'b0;
                else if (funct3 == 3'b111)
                    alu_result = alu_a % alu_b;
                else
                    alu_result = alu_a / alu_b;
            end
            default: alu_result = alu_a + alu_b;
        endcase
    end

    // ====== Branch Comparator ======
    wire branch_cond;
    assign branch_cond = is_compressed ?
                            (rvc_funct3 == 3'b110 ? (rf_rdata1 == 64'b0) :
                             rvc_funct3 == 3'b111 ? (rf_rdata1 != 64'b0) :
                             1'b0) :
                            (funct3 == 3'b000) ? (rf_rdata1 == rf_rdata2) :
                            (funct3 == 3'b001) ? (rf_rdata1 != rf_rdata2) :
                            (funct3 == 3'b100) ? ($signed(rf_rdata1) < $signed(rf_rdata2)) :
                            (funct3 == 3'b101) ? ($signed(rf_rdata1) >= $signed(rf_rdata2)) :
                            (funct3 == 3'b110) ? (rf_rdata1 < rf_rdata2) :
                            (funct3 == 3'b111) ? (rf_rdata1 >= rf_rdata2) :
                            1'b0;

    wire branch_taken = branch & branch_cond;

    // ====== Jump Target ======
    wire [63:0] jal_target  = pc + (is_compressed ? rvc_imm_j : imm_j);
    wire [63:0] jalr_target = (rf_rdata1 + (is_compressed ? 64'b0 : imm_i)) & ~64'h1;

    // ====== Program Counter Next ======
    assign pc_next = jump ? (jalr ? jalr_target : jal_target) :
                     branch_taken ? (pc + (is_compressed ? rvc_imm_b : imm_b)) :
                     pc + pc_inc;

    // ====== Data Memory Read ======
    reg [63:0] mem_rdata_word;
    always @(*) begin
        if (is_compressed && (rvc_q == 2'b10 && rvc_funct3 == 3'b010)) begin // C.LDSP
            mem_rdata_word = {dmem[dmem_addr+7], dmem[dmem_addr+6],
                              dmem[dmem_addr+5], dmem[dmem_addr+4],
                              dmem[dmem_addr+3], dmem[dmem_addr+2],
                              dmem[dmem_addr+1], dmem[dmem_addr]};
        end else if (is_compressed) begin
            if (rvc_q == 2'b00 && rvc_funct3 == 3'b011) begin // C.LD
                mem_rdata_word = {dmem[dmem_addr+7], dmem[dmem_addr+6],
                                  dmem[dmem_addr+5], dmem[dmem_addr+4],
                                  dmem[dmem_addr+3], dmem[dmem_addr+2],
                                  dmem[dmem_addr+1], dmem[dmem_addr]};
            end else begin // C.LW / C.LWSP
                mem_rdata_word = {{32{dmem[dmem_addr+3][7]}}, dmem[dmem_addr+3],
                                  dmem[dmem_addr+2], dmem[dmem_addr+1], dmem[dmem_addr]};
            end
        end else begin
            case (funct3)
                3'b000: mem_rdata_word = {{56{dmem[dmem_addr][7]}}, dmem[dmem_addr]};
                3'b001: mem_rdata_word = {{48{dmem[dmem_addr+1][7]}}, dmem[dmem_addr+1], dmem[dmem_addr]};
                3'b010: mem_rdata_word = {{32{dmem[dmem_addr+3][7]}}, dmem[dmem_addr+3], dmem[dmem_addr+2],
                                           dmem[dmem_addr+1], dmem[dmem_addr]};
                3'b011: mem_rdata_word = {dmem[dmem_addr+7], dmem[dmem_addr+6], dmem[dmem_addr+5],
                                           dmem[dmem_addr+4], dmem[dmem_addr+3], dmem[dmem_addr+2],
                                           dmem[dmem_addr+1], dmem[dmem_addr]};
                3'b100: mem_rdata_word = {56'b0, dmem[dmem_addr]};
                3'b101: mem_rdata_word = {48'b0, dmem[dmem_addr+1], dmem[dmem_addr]};
                3'b110: mem_rdata_word = {32'b0, dmem[dmem_addr+3], dmem[dmem_addr+2],
                                           dmem[dmem_addr+1], dmem[dmem_addr]};
                default: mem_rdata_word = {dmem[dmem_addr+7], dmem[dmem_addr+6], dmem[dmem_addr+5],
                                            dmem[dmem_addr+4], dmem[dmem_addr+3], dmem[dmem_addr+2],
                                            dmem[dmem_addr+1], dmem[dmem_addr]};
            endcase
        end
    end

    // ====== Register Write Data ======
    reg [63:0] reg_wdata;
    always @(*) begin
        if (csr_op) begin
            reg_wdata = csr_rdata;
        end else if (is_compressed) begin
            case (reg_src)
                2'b00: reg_wdata = alu_result;
                2'b01: reg_wdata = mem_rdata_word;
                2'b10: reg_wdata = pc + pc_inc;
                default: begin
                    if (rvc_q == 2'b01 && rvc_funct3 == 3'b010)
                        reg_wdata = rvc_imm;
                    else if (rvc_q == 2'b01 && rvc_funct3 == 3'b011 && rvc_rd != 5'b00010)
                        reg_wdata = rvc_imm_lui;
                    else if (rvc_q == 2'b10 && rvc_funct3 == 3'b100 && rvc_bit12 == 0 && rvc_rs2 != 0)
                        reg_wdata = rf_rdata2;
                    else
                        reg_wdata = rvc_imm;
                end
            endcase
        end else begin
            case (reg_src)
                2'b00: reg_wdata = alu_result;
                2'b01: reg_wdata = mem_rdata_word;
                2'b10: reg_wdata = pc + pc_inc;
                default: reg_wdata = imm_u;
            endcase
        end
    end

    // ====== Register File Write ======
    always @(posedge clk) begin
        if (reg_write && rvc_rd5 != 0)
            rf[rvc_rd5] <= reg_wdata;
    end

    // ====== Data Memory Write ======
    always @(posedge clk) begin
        if (mem_write) begin
            if (is_compressed && ((rvc_q == 2'b10 && rvc_funct3 == 3'b111) ||
                                  (rvc_q == 2'b00 && rvc_funct3 == 3'b111))) begin // C.SDSP / C.SD
                dmem[dmem_addr]   <= rf_rdata2[7:0];
                dmem[dmem_addr+1] <= rf_rdata2[15:8];
                dmem[dmem_addr+2] <= rf_rdata2[23:16];
                dmem[dmem_addr+3] <= rf_rdata2[31:24];
                dmem[dmem_addr+4] <= rf_rdata2[39:32];
                dmem[dmem_addr+5] <= rf_rdata2[47:40];
                dmem[dmem_addr+6] <= rf_rdata2[55:48];
                dmem[dmem_addr+7] <= rf_rdata2[63:56];
            end else if (is_compressed) begin // C.SWSP / C.SW
                dmem[dmem_addr]   <= rf_rdata2[7:0];
                dmem[dmem_addr+1] <= rf_rdata2[15:8];
                dmem[dmem_addr+2] <= rf_rdata2[23:16];
                dmem[dmem_addr+3] <= rf_rdata2[31:24];
            end else begin
                case (funct3)
                    3'b000: begin
                        dmem[dmem_addr]   <= rf_rdata2[7:0];
                    end
                    3'b001: begin
                        dmem[dmem_addr]   <= rf_rdata2[7:0];
                        dmem[dmem_addr+1] <= rf_rdata2[15:8];
                    end
                    3'b010: begin
                        dmem[dmem_addr]   <= rf_rdata2[7:0];
                        dmem[dmem_addr+1] <= rf_rdata2[15:8];
                        dmem[dmem_addr+2] <= rf_rdata2[23:16];
                        dmem[dmem_addr+3] <= rf_rdata2[31:24];
                    end
                    3'b011: begin
                        dmem[dmem_addr]   <= rf_rdata2[7:0];
                        dmem[dmem_addr+1] <= rf_rdata2[15:8];
                        dmem[dmem_addr+2] <= rf_rdata2[23:16];
                        dmem[dmem_addr+3] <= rf_rdata2[31:24];
                        dmem[dmem_addr+4] <= rf_rdata2[39:32];
                        dmem[dmem_addr+5] <= rf_rdata2[47:40];
                        dmem[dmem_addr+6] <= rf_rdata2[55:48];
                        dmem[dmem_addr+7] <= rf_rdata2[63:56];
                    end
                endcase
            end
        end
    end

endmodule
