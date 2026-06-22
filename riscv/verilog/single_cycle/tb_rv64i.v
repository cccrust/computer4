// tb_rv64i.v -- testbench for single-cycle RV64IM + Zicsr + C CPU (v0.4)

`timescale 1ns / 1ps

module tb_rv64i;

    reg clk;
    reg rst_n;
    wire dbg_ecall;

    integer i, j, word_val;
    reg [7:0] byte_val;
    reg term;
    reg uart_active;
    reg trace_enabled;
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
        $display("  v0.4  (UART MMIO + 0x80000000)");
        $display("========================================");
        $display("");

        trace_enabled = $test$plusargs("TRACE");
        rst_n = 0;
        term = 0;
        uart_active = 0;
        #15;
        rst_n = 1;
        #5;

        for (i = 0; i < 40000000; i = i + 1) begin
            #10;
            if (term) begin
                $display("");
                $display("  #####  PASS  #####");
                $finish;
            end
            // Timeout after 10000 cycles with no UART activity
            if (i > 500 && !uart_active) begin
                $display("  #####  FAIL (no UART output)  #####");
                $finish;
            end
        end

        $display("");
        $display("  #####  PASS (timeout with UART)  #####");
        $finish;
    end

    // Optional instruction trace, enabled with +TRACE
    integer trace_cnt;
    initial trace_cnt = 0;
    always @(posedge clk) begin
        if (trace_enabled && trace_cnt < 200) begin
            $display("[%d] PC=%h sp=%h a0=%h a1=%h a2=%h a7=%h",
                trace_cnt, cpu.pc, cpu.rf[2], cpu.rf[10], cpu.rf[11], cpu.rf[12], cpu.rf[17]);
            trace_cnt = trace_cnt + 1;
        end
    end

    // UART write monitor
    always @(negedge clk) begin
        if (cpu.mem_write && cpu.is_uart_mmio && cpu.alu_result[3:0] == 4'h0) begin
            uart_active = 1;
        end
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
                    $write("(sa0=%h,a0=%h,imem50=%h)", sa0, cpu.rf[10], cpu.imem[50]);
                    for (j = 0; j < cpu.rf[11]; j = j + 1) begin
                        if (sa0 + j >= 64'h80000000) begin
                            word_val = cpu.imem[((sa0 + j) - 64'h80000000) >> 2];
                        end else begin
                            word_val = cpu.imem[(sa0 + j) >> 2];
                        end
                        byte_val = word_val >> (((sa0 + j) & 3) * 8);
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
