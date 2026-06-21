// tb_rv64i.v -- testbench for single-cycle RV64IM + Zicsr + C CPU (v0.3)

`timescale 1ns / 1ps

module tb_rv64i;

    reg clk;
    reg rst_n;
    wire dbg_ecall;

    integer i, j, word_val;
    reg [7:0] byte_val;
    reg term;
    reg [63:0] sa0, sa7;

    rv64i_cpu cpu (
        .clk(clk),
        .rst_n(rst_n),
        .dbg_ecall(dbg_ecall)
    );

    initial begin
        clk = 0;
        forever #5 clk = ~clk;
    end

    initial begin
        for (i = 0; i < 32; i = i + 1)
            cpu.rf[i] = 64'b0;
    end

    initial begin
        $display("========================================");
        $display("  RV64IM + Zicsr + C Single-Cycle CPU");
        $display("========================================");
        $display("");

        rst_n = 0;
        term = 0;
        #15;
        rst_n = 1;
        #5;

        for (i = 0; i < 5000; i = i + 1) begin
            #10;
            if (term) begin
                $display("");
                $display("--- Final Register State ---");
                $display("pc  = 0x%h", cpu.pc);
                $display("x1  = %0d", cpu.rf[1]);
                $display("x2  = %0d", cpu.rf[2]);
                $display("x10 = %0d", cpu.rf[10]);
                $display("x17 = %0d (a7)", cpu.rf[17]);
                $display("");
                $display("  #####  PASS  #####");
                $finish;
            end
        end

        $display("");
        $display("  #####  FAIL (timeout)  #####");
        $finish;
    end

    // ECALL syscall handler (monitor on negedge so combinational dbg_ecall is stable)
    always @(negedge clk) begin
        if (cpu.dbg_ecall) begin
            sa7 = cpu.rf[17];
            sa0 = cpu.rf[10];
            case (sa7)
                64'd0: begin // exit
                    $write("[exit a0=%0d]", sa0);
                    term = 1;
                end
                64'd1: begin // putchar
                    $write("%c", sa0);
                    $fflush();
                end
                64'd2: begin // puts
                    $write("puts[%0d]:", cpu.rf[11]);
                    for (j = 0; j < cpu.rf[11]; j = j + 1) begin
                        if (sa0 + j >= 64'h8000) begin
                            byte_val = cpu.dmem[(sa0 + j) & 13'h1FFF];
                            $write("d%02x", byte_val);
                        end else begin
                            word_val = cpu.imem[(sa0 + j) >> 2];
                            byte_val = word_val >> (((sa0 + j) & 3) * 8);
                            $write("i%02x", byte_val);
                        end
                        $write("%c", byte_val);
                    end
                    $write("\n");
                    $fflush();
                end
                default: begin
                    $display("[ecall unknown a7=%0d a0=%0d]", sa7, sa0);
                end
            endcase
        end
    end

endmodule
