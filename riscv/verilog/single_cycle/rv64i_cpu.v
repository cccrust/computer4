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

module rv64i_cpu #(parameter BASE_ADDR = 64'h80000000) (
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
            pc <= BASE_ADDR;
        else
            pc <= pc_next;
    end

    // ====== Unified Memory (32-bit words, 256KB) ======
    reg [31:0] imem [0:16383];

    wire [13:0] word_addr = pc[15:2];
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
        $readmemh("program.hex", imem);
    end

    // ====== Compressed (16-bit) / Uncompressed (32-bit) Decode ======
    wire [6:0] opcode  = instr[6:0];
    wire [4:0] rd      = instr[11:7];
    wire [2:0] funct3  = instr[14:12];
    wire [4:0] rs1     = instr[19:15];
    wire [4:0] rs2     = instr[24:20];
    wire [6:0] funct7  = instr[31:25];
    wire [4:0] funct5  = instr[31:27];

    wire is_lr = !is_compressed && opcode == 7'b0101111 && funct5 == 5'b00010 && funct3 == 3'b010;
    wire is_sc = !is_compressed && opcode == 7'b0101111 && funct5 == 5'b00011 && funct3 == 3'b010;

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
                              (rvc_q == 2'b10 && (rvc_funct3 == 3'b010 || rvc_funct3 == 3'b011 || rvc_funct3 == 3'b110 || rvc_funct3 == 3'b111)) ? 5'b00010 : // Q=10 SP-relative: LWSP/LDSP/SWSP/SDSP use sp
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

    wire [6:0] rvc_uimm_lw = {fetch_half[5], fetch_half[12], fetch_half[11:10], fetch_half[6], 2'b00};
    wire [63:0] rvc_imm_lw = {57'b0, rvc_uimm_lw};

    wire [63:0] rvc_imm_sw = rvc_imm_lw;

    wire [9:0] rvc_imm_addi16sp_val = {fetch_half[12], fetch_half[4:3],
                                        fetch_half[5], fetch_half[2],
                                        fetch_half[6], 4'b0000};
    wire [63:0] rvc_imm_addi16sp = {{54{rvc_imm_addi16sp_val[9]}}, rvc_imm_addi16sp_val};

    wire [63:0] rvc_imm_lui = {{43{rvc_imm_i6[5]}}, rvc_imm_i6, 12'b0};

    // Q=10 SP-relative: stores use bits[12:7], loads use bits[6:2]; all byte offsets (no scaling)
    wire [5:0] rvc_uimm_sp_ld = {fetch_half[12], fetch_half[6:2]};
    wire [63:0] rvc_imm_sp_ld = {58'b0, rvc_uimm_sp_ld};
    wire [5:0] rvc_uimm_sp_st = {fetch_half[12], fetch_half[11:7]};
    wire [63:0] rvc_imm_sp_st = {58'b0, rvc_uimm_sp_st};

    // ====== Register File ======
    reg [63:0] rf [0:31];
    wire [63:0] rf_rdata1 = (rvc_rs15 != 0) ? rf[rvc_rs15] : 64'b0;
    wire [63:0] rf_rdata2 = (rvc_rs25 != 0) ? rf[rvc_rs25] : 64'b0;

    // ====== Data Memory (shared with imem, unified 256KB) ======
    wire [13:0] data_word_addr = alu_result[15:2];
    wire [1:0]  byte_lane = alu_result[1:0];
    wire [31:0] data_word = imem[data_word_addr];
    wire        is_sd = is_compressed ?
                           ((rvc_q == 2'b10 && rvc_funct3 == 3'b111) ||   // C.SDSP
                            (rvc_q == 2'b00 && rvc_funct3 == 3'b011)) :    // C.SD (RV64) / C.FSW (RV32)
                           (opcode == 7'b0100011 && funct3 == 3'b011);     // SD (S-type)
    wire        is_ram_addr = (alu_result >= BASE_ADDR && alu_result < BASE_ADDR + 262144);
    wire        is_uart_mmio = (alu_result >= 32'h10000000 && alu_result < 32'h10000010);
    wire        is_clint_mmio = (alu_result >= 32'h02000000 && alu_result < 32'h02010000);
    wire        is_mmio = is_uart_mmio || is_clint_mmio;

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
    reg [63:0] mie;
    reg [63:0] mtvec;
    reg [63:0] mscratch;
    reg [63:0] mepc;
    reg [63:0] mcause;

    // ====== CLINT Timer Registers ======
    reg [63:0] clint_mtime;
    reg [63:0] clint_mtimecmp;

    wire timer_irq = mie[7] && (clint_mtime >= clint_mtimecmp);

    reg [63:0]  csr_rdata;
    reg         csr_write;
    reg [63:0]  csr_wdata;

    always @(*) begin
        case (instr[31:20])
            12'h300: csr_rdata = mstatus;
            12'h304: csr_rdata = mie;
            12'h305: csr_rdata = mtvec;
            12'h340: csr_rdata = mscratch;
            12'h341: csr_rdata = mepc;
            12'h342: csr_rdata = mcause;
            12'hB00: csr_rdata = mcycle;
            12'hB02: csr_rdata = minstret;
            default: csr_rdata = 64'b0;
        endcase
    end

    // ====== ECALL / MRET / WFI Detection ======
    wire [11:0] funct12 = instr[31:20];
    wire is_ecall = !is_compressed && opcode == 7'b1110011 && funct3 == 3'b000 && funct12 == 12'h000;
    wire is_mret  = !is_compressed && opcode == 7'b1110011 && funct3 == 3'b000 && funct12 == 12'h302;
    wire is_wfi   = !is_compressed && opcode == 7'b1110011 && funct3 == 3'b000 && funct12 == 12'h105;

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
                    3'b001: begin // C.FLWSP / reserved
                        reg_write = (rvc_rd != 0);
                        alu_src_a = 1'b0; alu_src_b = 2'b01;
                        reg_src = 2'b01;
                    end
                    3'b010: begin // C.LWSP
                        reg_write = (rvc_rd != 0);
                        alu_src_a = 1'b0; alu_src_b = 2'b01;
                        reg_src = 2'b01;
                    end
                    3'b011: begin // C.LDSP (RV64)
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
                    3'b001: begin // C.FLW (RV32) / C.LD (RV64)
                        reg_write = 1'b1;
                        alu_src_a = 1'b0; alu_src_b = 2'b01;
                        reg_src = 2'b01;
                    end
                    3'b010: begin // C.LW
                        reg_write = 1'b1;
                        alu_src_a = 1'b0; alu_src_b = 2'b01;
                        reg_src = 2'b01;
                    end
                    3'b011: begin // C.FSW (RV32) / C.SD (RV64)
                        mem_write = 1'b1;
                        alu_src_a = 1'b0; alu_src_b = 2'b01;
                    end
                    3'b110: begin // C.SW
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
                7'b0101111: begin // A-extension: lr.w / sc.w
                    if (funct5 == 5'b00010) begin // lr.w
                        reg_write = 1'b1;
                        reg_src = 2'b01;
                        alu_src_a = 1'b0; alu_src_b = 2'b01;
                    end else if (funct5 == 5'b00011) begin // sc.w
                        reg_write = 1'b1;
                        mem_write = 1'b1;
                        alu_src_a = 1'b0; alu_src_b = 2'b01;
                    end
                end
                7'b1110011: begin
                    if (!is_ecall && !is_mret && !is_wfi) begin
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
                             rvc_funct3 == 3'b001 ? rvc_imm_ld :   // C.LD (RV64) / C.FLW (RV32)
                             rvc_funct3 == 3'b010 ? rvc_imm_lw :   // C.LW
                             rvc_funct3 == 3'b011 ? rvc_imm_ld :   // C.SD (RV64) / C.FSW (RV32)
                             rvc_funct3 == 3'b110 ? rvc_imm_sw :   // C.SW
                             rvc_imm) :
                         (rvc_q == 2'b01) ?
                            (rvc_funct3 == 3'b011 && rvc_rd == 5'b00010 ? rvc_imm_addi16sp :
                             rvc_funct3 == 3'b100 && fetch_half[11:10] == 2'b10 ? {{58{fetch_half[12]}}, fetch_half[12], fetch_half[6:2]} :
                             rvc_funct3 == 3'b100 && (fetch_half[11:10] == 2'b00) ? {59'b0, fetch_half[12], fetch_half[6:2]} :
                             rvc_imm) :
                         (rvc_q == 2'b10) ?
                             (rvc_funct3 == 3'b000 ? {59'b0, rvc_rd[4:0]} :
                              rvc_funct3 == 3'b001 ? rvc_imm_sp_ld :
                              rvc_funct3 == 3'b010 ? rvc_imm_sp_ld :
                              rvc_funct3 == 3'b011 ? rvc_imm_sp_ld :
                              rvc_funct3 == 3'b110 ? rvc_imm_sp_st :
                              rvc_funct3 == 3'b111 ? rvc_imm_sp_st :
                             rvc_imm) :
                         64'b0;

    always @(*) begin
        if (is_compressed) begin
            case (alu_src_b)
                2'b00: alu_b = rf_rdata2;
                default: alu_b = rvc_alu_imm;
            endcase
        end else if (opcode == 7'b0101111) begin
            alu_b = 64'b0; // AMO: base address only (no offset)
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
            // FIX: use funct3 only for logical/arithmetic ops (funct7 overlaps with immediate)
            case (funct3)
                3'b000: alu_ctrl = `ALU_ADD;        // ADDI
                3'b001: alu_ctrl = (funct7[5] == 1'b0) ? `ALU_SLL : `ALU_ADD; // SLLI
                3'b010: alu_ctrl = `ALU_SLT;         // SLTI
                3'b011: alu_ctrl = `ALU_SLTU;        // SLTIU
                3'b100: alu_ctrl = `ALU_XOR;         // XORI
                3'b101: alu_ctrl = funct7[5] ? `ALU_SRA : `ALU_SRL; // SRLI/SRAI
                3'b110: alu_ctrl = `ALU_OR;          // ORI
                3'b111: alu_ctrl = `ALU_AND;         // ANDI
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
            mie      <= 64'b0;
            mtvec    <= 64'b0;
            mscratch <= 64'b0;
            mepc     <= 64'b0;
            mcause   <= 64'b0;
            clint_mtime    <= 64'b0;
            clint_mtimecmp <= ~64'b0; // never fire by default
        end else begin
            mcycle <= mcycle + 1;
            minstret <= minstret + 1;
            clint_mtime <= clint_mtime + 1;
            if (is_ecall) begin
                mepc   <= pc;
                mcause <= 64'd11;
            end
            if (is_mret) begin
                mstatus[3] <= mstatus[7]; // MIE <= MPIE
                mstatus[7] <= 1'b1;       // MPIE <= 1
            end
            if (!is_mret && !is_ecall && timer_irq && mstatus[3]) begin
                mepc   <= pc + pc_inc;
                mcause <= 64'h8000000000000007;
                mstatus[7] <= mstatus[3]; // MPIE <= old MIE
                mstatus[3] <= 1'b0;       // MIE <= 0 (disable interrupts)
            end
            if (csr_write && !is_ecall && !is_mret) begin
                case (instr[31:20])
                    12'h300: mstatus  <= csr_wdata;
                    12'h304: mie      <= csr_wdata;
                    12'h305: mtvec    <= csr_wdata;
                    12'h340: mscratch <= csr_wdata;
                    12'h341: mepc     <= csr_wdata;
                    12'h342: mcause   <= csr_wdata;
                    12'hB00: mcycle   <= csr_wdata;
                    12'hB02: minstret <= csr_wdata;
                endcase
            end
            if (mem_write && is_clint_mmio) begin
                case (alu_result[15:0])
                    16'h4000: clint_mtimecmp <= rf_rdata2;
                    16'h4004: clint_mtimecmp[63:32] <= rf_rdata2[31:0];
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
    wire [63:0] pc_regular = jump ? (jalr ? jalr_target : jal_target) :
                              branch_taken ? (pc + (is_compressed ? rvc_imm_b : imm_b)) :
                              pc + pc_inc;

    assign pc_next = is_mret ? mepc :
                     (timer_irq && mstatus[3] && !is_ecall) ? mtvec :
                     pc_regular;

    // ====== MMIO Read (UART + CLINT) ======
    reg [63:0] mmio_rdata;
    always @(*) begin
        mmio_rdata = 64'b0;
        if (is_uart_mmio) begin
            case (alu_result[3:0])
                4'h0: mmio_rdata = 64'b0;               // RHR (no input)
                4'h5: mmio_rdata = 64'h60;              // LSR (THR empty + TEMT)
            endcase
        end else if (is_clint_mmio) begin
            case (alu_result[15:0])
                16'hBFF8: mmio_rdata = clint_mtime;
                16'hBFFC: mmio_rdata = {32'b0, clint_mtime[63:32]};
                16'h4000: mmio_rdata = clint_mtimecmp;
            endcase
        end
    end

    // ====== Data Memory Read (from unified imem) ======
    reg [63:0] mem_rdata_word;
    always @(*) begin
        if (is_mmio) begin
            mem_rdata_word = mmio_rdata;
        end else begin
            if (is_compressed && (rvc_q == 2'b10 && rvc_funct3 == 3'b010)) begin // C.LWSP
                mem_rdata_word = {{32{data_word[31]}}, data_word};
            end else if (is_compressed && (rvc_q == 2'b10 && rvc_funct3 == 3'b011)) begin // C.LDSP (RV64)
                mem_rdata_word = {imem[data_word_addr+1], imem[data_word_addr]};
            end else if (is_compressed && rvc_q == 2'b00 && rvc_funct3 == 3'b001) begin // C.LD (Q=00)
                mem_rdata_word = {imem[data_word_addr+1], imem[data_word_addr]};
            end else if (is_compressed) begin // C.LW / C.LWSP
                mem_rdata_word = {{32{data_word[31]}}, data_word};
            end else begin
                case (funct3)
                    3'b000: begin // LB
                        if      (byte_lane == 2'b00) mem_rdata_word = {{56{data_word[7]}},  data_word[7:0]};
                        else if (byte_lane == 2'b01) mem_rdata_word = {{56{data_word[15]}}, data_word[15:8]};
                        else if (byte_lane == 2'b10) mem_rdata_word = {{56{data_word[23]}}, data_word[23:16]};
                        else                        mem_rdata_word = {{56{data_word[31]}}, data_word[31:24]};
                    end
                    3'b001: begin // LH
                        if (byte_lane[1]) mem_rdata_word = {{48{data_word[31]}}, data_word[31:16]};
                        else              mem_rdata_word = {{48{data_word[15]}}, data_word[15:0]};
                    end
                    3'b010: mem_rdata_word = {{32{data_word[31]}}, data_word};                // LW
                    3'b011: mem_rdata_word = {imem[data_word_addr+1], data_word};             // LD
                    3'b100: begin // LBU
                        if      (byte_lane == 2'b00) mem_rdata_word = {56'b0, data_word[7:0]};
                        else if (byte_lane == 2'b01) mem_rdata_word = {56'b0, data_word[15:8]};
                        else if (byte_lane == 2'b10) mem_rdata_word = {56'b0, data_word[23:16]};
                        else                        mem_rdata_word = {56'b0, data_word[31:24]};
                    end
                    3'b101: begin // LHU
                        if (byte_lane[1]) mem_rdata_word = {48'b0, data_word[31:16]};
                        else              mem_rdata_word = {48'b0, data_word[15:0]};
                    end
                    3'b110: mem_rdata_word = {32'b0, data_word};                              // LWU
                    default: mem_rdata_word = {imem[data_word_addr+1], data_word};            // LD
                endcase
            end
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
        if (reg_write && rvc_rd5 != 0) begin
            if (is_sc)
                rf[rvc_rd5] <= 64'b0; // sc.w always succeeds in single core
            else begin
                rf[rvc_rd5] <= reg_wdata;
            end
        end
    end

    // ====== Data Memory Write (to unified imem) ======
    // Comb: construct word to write into imem[data_word_addr]
    reg [31:0] mem_wdata;
    always @(*) begin
        mem_wdata = imem[data_word_addr]; // default: read-modify-write
        if (is_sd) begin
            mem_wdata = rf_rdata2[31:0];
        end else begin
            case (funct3)
                3'b000: begin
                    if      (byte_lane == 2'b00) mem_wdata = {imem[data_word_addr][31:8], rf_rdata2[7:0]};
                    else if (byte_lane == 2'b01) mem_wdata = {imem[data_word_addr][31:16], rf_rdata2[7:0], imem[data_word_addr][7:0]};
                    else if (byte_lane == 2'b10) mem_wdata = {imem[data_word_addr][31:24], rf_rdata2[7:0], imem[data_word_addr][15:0]};
                    else                         mem_wdata = {rf_rdata2[7:0], imem[data_word_addr][23:0]};
                end
                3'b001: begin
                    if (byte_lane[1]) mem_wdata = {rf_rdata2[15:0], imem[data_word_addr][15:0]};
                    else              mem_wdata = {imem[data_word_addr][31:16], rf_rdata2[15:0]};
                end
                default: mem_wdata = rf_rdata2[31:0]; // SW / (SD handled above)
            endcase
        end
    end

    always @(posedge clk) begin
        if (mem_write && is_uart_mmio && alu_result[3:0] == 4'h0) begin
            if (rf_rdata2[7:0] >= 8'h80) begin
                $display("[UART_BAD] pc=%h a0=%h byte=%02h", pc, rf_rdata2, rf_rdata2[7:0]);
            end
            $write("%c", rf_rdata2[7:0]);
            $fflush();
        end else if (mem_write && !is_mmio) begin
            imem[data_word_addr] <= mem_wdata;
            if (is_sd)
                imem[data_word_addr+1] <= rf_rdata2[63:32];
        end
    end

endmodule
