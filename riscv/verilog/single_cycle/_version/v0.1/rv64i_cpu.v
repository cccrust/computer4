// rv64i_cpu.v — RISC-V 64I 單週期處理器

`define ALU_ADD  4'b0000
`define ALU_SUB  4'b0001
`define ALU_SLL  4'b0010
`define ALU_SLT  4'b0011
`define ALU_SLTU 4'b0100
`define ALU_XOR  4'b0101
`define ALU_SRL  4'b0110
`define ALU_SRA  4'b0111
`define ALU_OR   4'b1000
`define ALU_AND  4'b1001
`define ALU_PASS 4'b1010

module rv64i_cpu (
    input  wire       clk,
    input  wire       rst_n
);

    // ====== Program Counter ======
    reg [63:0] pc;
    wire [63:0] pc_plus4 = pc + 64'd4;
    wire [63:0] pc_next;

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n)
            pc <= 64'b0;
        else
            pc <= pc_next;
    end

    // ====== Instruction Memory ======
    reg [31:0] imem [0:1023];
    wire [31:0] instr = imem[pc[11:2]];

    integer init_i;
    initial begin
        for (init_i = 0; init_i < 1024; init_i = init_i + 1)
            imem[init_i] = 32'h00000013;
        $readmemh("program.hex", imem);
    end

    // ====== Instruction Decode ======
    wire [6:0] opcode = instr[6:0];
    wire [4:0] rd     = instr[11:7];
    wire [2:0] funct3 = instr[14:12];
    wire [4:0] rs1    = instr[19:15];
    wire [4:0] rs2    = instr[24:20];
    wire [6:0] funct7 = instr[31:25];

    // ====== Register File ======
    reg [63:0] rf [0:31];
    wire [63:0] rf_rdata1 = (rs1 != 0) ? rf[rs1] : 64'b0;
    wire [63:0] rf_rdata2 = (rs2 != 0) ? rf[rs2] : 64'b0;

    // ====== Data Memory (byte-addressable, 8KB) ======
    reg [7:0] dmem [0:8191];
    wire [12:0] dmem_addr = alu_result[12:0];

    // ====== Immediate Generator ======
    wire [63:0] imm_i = {{52{instr[31]}}, instr[31:20]};
    wire [63:0] imm_s = {{52{instr[31]}}, instr[31:25], instr[11:7]};
    wire [63:0] imm_b = {{51{instr[31]}}, instr[31], instr[7], instr[30:25], instr[11:8], 1'b0};
    wire [63:0] imm_u = {instr[31:12], 12'b0};
    wire [63:0] imm_j = {{43{instr[31]}}, instr[31], instr[19:12], instr[20], instr[30:21], 1'b0};

    // ====== Control Unit ======
    reg        reg_write;
    reg        mem_write;
    reg        branch;
    reg        jump;
    reg        jalr;
    reg [1:0]  reg_src;
    reg        alu_src_a;
    reg [1:0]  alu_src_b;

    always @(*) begin
        reg_write = 1'b0; mem_write = 1'b0;
        branch = 1'b0; jump = 1'b0; jalr = 1'b0;
        reg_src = 2'b00; alu_src_a = 1'b0; alu_src_b = 1'b0;

        case (opcode)
            7'b0110011: begin // R-type
                reg_write = 1'b1; alu_src_a = 1'b0; alu_src_b = 2'b00;
                reg_src = 2'b00;
            end
            7'b0010011: begin // I-type ALU
                reg_write = 1'b1; alu_src_a = 1'b0; alu_src_b = 2'b01;
                reg_src = 2'b00;
            end
            7'b0000011: begin // Load
                reg_write = 1'b1;
                alu_src_a = 1'b0; alu_src_b = 2'b01;
                reg_src = 2'b01;
            end
            7'b0100011: begin // Store
                mem_write = 1'b1;
                alu_src_a = 1'b0; alu_src_b = 2'b01;
            end
            7'b1100011: begin // Branch
                branch = 1'b1;
                alu_src_a = 1'b0; alu_src_b = 2'b00;
            end
            7'b0110111: begin // LUI
                reg_write = 1'b1;
                reg_src = 2'b11;
            end
            7'b0010111: begin // AUIPC
                reg_write = 1'b1;
                alu_src_a = 1'b1; alu_src_b = 2'b10;
                reg_src = 2'b00;
            end
            7'b1101111: begin // JAL
                reg_write = 1'b1; jump = 1'b1;
                reg_src = 2'b10;
            end
            7'b1100111: begin // JALR
                reg_write = 1'b1; jump = 1'b1; jalr = 1'b1;
                alu_src_a = 1'b0; alu_src_b = 2'b01;
                reg_src = 2'b10;
            end
        endcase
    end

    // ====== ALU ======
    wire [63:0] alu_a;
    reg  [63:0] alu_b;
    reg [3:0] alu_ctrl;

    assign alu_a = alu_src_a ? pc : rf_rdata1;

    always @(*) begin
        case (alu_src_b)
            2'b00: alu_b = rf_rdata2;
            2'b01: alu_b = (opcode == 7'b0100011) ? imm_s : imm_i;
            2'b10: alu_b = imm_u;
            2'b11: alu_b = 64'd4;
            default: alu_b = rf_rdata2;
        endcase
    end

    always @(*) begin
        if (opcode == 7'b0110011 || opcode == 7'b0010011) begin
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

    reg [63:0] alu_result;
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
            default:   alu_result = alu_a + alu_b;
        endcase
    end

    // ====== Branch Comparator ======
    wire branch_cond;
    assign branch_cond = (funct3 == 3'b000) ? (rf_rdata1 == rf_rdata2) :
                         (funct3 == 3'b001) ? (rf_rdata1 != rf_rdata2) :
                         (funct3 == 3'b100) ? ($signed(rf_rdata1) < $signed(rf_rdata2)) :
                         (funct3 == 3'b101) ? ($signed(rf_rdata1) >= $signed(rf_rdata2)) :
                         (funct3 == 3'b110) ? (rf_rdata1 < rf_rdata2) :
                         (funct3 == 3'b111) ? (rf_rdata1 >= rf_rdata2) :
                         1'b0;

    wire branch_taken = branch & branch_cond;

    // ====== Jump Target ======
    wire [63:0] jal_target  = pc + imm_j;
    wire [63:0] jalr_target = (rf_rdata1 + imm_i) & ~64'h1;

    // ====== Program Counter Next ======
    assign pc_next = jump ? (jalr ? jalr_target : jal_target) :
                     branch_taken ? (pc + imm_b) :
                     pc_plus4;

    // ====== Data Memory Read ======
    reg [63:0] mem_rdata_word;
    always @(*) begin
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

    // ====== Register Write-back ======
    wire [63:0] reg_wdata;
    assign reg_wdata = (reg_src == 2'b00) ? alu_result :
                       (reg_src == 2'b01) ? mem_rdata_word :
                       (reg_src == 2'b10) ? pc_plus4 :
                       imm_u;

    // ====== Register File Write ======
    always @(posedge clk) begin
        if (reg_write && rd != 0)
            rf[rd] <= reg_wdata;
    end

    // ====== Data Memory Write (no LHS concatenation) ======
    always @(posedge clk) begin
        if (mem_write) begin
            case (funct3)
                3'b000: dmem[dmem_addr] <= rf_rdata2[7:0];
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

endmodule
