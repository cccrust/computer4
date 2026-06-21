// tb_rv64i.v -- testbench for single-cycle RV64I CPU

`timescale 1ns / 1ps

module tb_rv64i;

    reg clk;
    reg rst_n;

    integer i;

    rv64i_cpu cpu (
        .clk(clk),
        .rst_n(rst_n)
    );

    initial begin
        clk = 0;
        forever #5 clk = ~clk;
    end

    initial begin
        $display("========================================");
        $display("  RV64I Single-Cycle CPU Test");
        $display("========================================");
        $display("");

        rst_n = 0;
        #15;
        rst_n = 1;
        #5;

        for (i = 0; i < 30; i = i + 1) begin
            #10;
        end

        $display("--- Register File ---");
        $display("x1  = %0d (expect 42)",             cpu.rf[1]);
        $display("x2  = %0d (expect 58)",             cpu.rf[2]);
        $display("x3  = %0d (expect 100)",            cpu.rf[3]);
        $display("x4  = %0d (expect 100)",            cpu.rf[4]);
        $display("x5  = %0d (expect 58)",             cpu.rf[5]);
        $display("x6  = %0d (expect 0)",              cpu.rf[6]);
        $display("x7  = %0d (expect 5)",              cpu.rf[7]);
        $display("x8  = %0d (expect 10)",             cpu.rf[8]);
        $display("x9  = %0d (expect 1)",              cpu.rf[9]);
        $display("x10 = 0x%h (expect 0x12345000)",    cpu.rf[10]);
        $display("x11 = 0x%h (expect 0x40)",          cpu.rf[11]);
        $display("x28 = %0d (expect 11)",             cpu.rf[28]);

        $display("");
        $display("--- Data Memory ---");
        $display("mem[0..7] = 0x%h 0x%h 0x%h 0x%h 0x%h 0x%h 0x%h 0x%h",
                 cpu.dmem[7], cpu.dmem[6], cpu.dmem[5], cpu.dmem[4],
                 cpu.dmem[3], cpu.dmem[2], cpu.dmem[1], cpu.dmem[0]);

        $display("");

        if (cpu.rf[28] === 64'd11 &&
            cpu.rf[3]  === 64'd100 &&
            cpu.rf[4]  === 64'd100 &&
            cpu.rf[10] === 64'h12345000 &&
            cpu.rf[6]  === 64'd0) begin
            $display("  #####  PASS  #####");
        end else begin
            $display("  #####  FAIL  #####");
        end

        $display("");
        $display("========================================");
        $finish;
    end

endmodule
