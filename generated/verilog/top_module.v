/* Automatically generated by Ferrum HDL. */

module top_module
(
    // Inputs
    input wire clk,
    input wire rst,
    // Outputs
    output wire seg$0,
    output wire seg$1,
    output wire seg$2,
    output wire seg$3,
    output wire anodes$0,
    output wire anodes$1,
    output wire anodes$2,
    output wire anodes$3,
    output wire anodes$4,
    output wire anodes$5,
    output wire anodes$6,
    output wire dp
);

    reg [2:0] dff;
    initial begin
        dff = 3'd0;
    end
    wire [2:0] __tmp_2;
    always @(posedge clk or posedge rst) begin
        if (rst)
            dff <= 3'd0;
        else
            dff <= __tmp_2;
    end

    wire [1:0] mux;
    Idx$succ __Idx$succ (
        // Inputs
        .self$(dff[1 +: 2]),
        // Outputs
        .mux(mux)
    );

    assign __tmp_2 = {
        mux,
        dff[1 +: 2] == 2'd3
    };

    wire [1:0] rr;
    RoundRobin$signal __RoundRobin$signal (
        // Inputs
        .clk(clk),
        .rst(rst),
        .next(dff[0]),
        // Outputs
        .rr(rr)
    );

    RoundRobin$selector __RoundRobin$selector (
        // Inputs
        .self$(rr),
        // Outputs
        .selector_out(seg$0),
        .selector_out_1(seg$1),
        .selector_out_2(seg$2),
        .selector_out_3(seg$3)
    );

    wire [3:0] mux_out;
    RoundRobin$mux __RoundRobin$mux (
        // Inputs
        .self$(rr),
        .inputs$0(4'd1),
        .inputs$1(4'd2),
        .inputs$2(4'd3),
        .inputs$3(4'd4),
        // Outputs
        .mux_out(mux_out)
    );

    SSDisplay$encode __SSDisplay$encode (
        // Inputs
        .n(mux_out),
        // Outputs
        .a(anodes$0),
        .b(anodes$1),
        .c(anodes$2),
        .d(anodes$3),
        .e(anodes$4),
        .f(anodes$5),
        .g(anodes$6)
    );

    assign dp = 0;

endmodule

module Idx$succ
(
    // Inputs
    input wire [1:0] self$,
    // Outputs
    output wire [1:0] mux
);

    always @(*) begin
        case (self$ == 2'd3)
            1'h1:
                mux = 2'd0;
            default:
                mux = self$ + 2'd1;
        endcase
    end

endmodule

module RoundRobin$signal
(
    // Inputs
    input wire clk,
    input wire rst,
    input wire next,
    // Outputs
    output reg [1:0] rr
);

    initial begin
        rr = 2'd0;
    end
    wire [1:0] next_out;
    always @(posedge clk or posedge rst) begin
        if (rst)
            rr <= 2'd0;
        else if (next)
            rr <= next_out;
    end

    RoundRobin$next __RoundRobin$next (
        // Inputs
        .self$(rr),
        // Outputs
        .next_out(next_out)
    );

endmodule

module RoundRobin$next
(
    // Inputs
    input wire [1:0] self$,
    // Outputs
    output wire [1:0] next_out
);

    Idx$succ_1 __Idx$succ (
        // Inputs
        .self$(self$),
        // Outputs
        .mux(next_out)
    );

endmodule

module Idx$succ_1
(
    // Inputs
    input wire [1:0] self$,
    // Outputs
    output wire [1:0] mux
);

    always @(*) begin
        case (self$ == 2'd3)
            1'h1:
                mux = 2'd0;
            default:
                mux = self$ + 2'd1;
        endcase
    end

endmodule

module RoundRobin$selector
(
    // Inputs
    input wire [1:0] self$,
    // Outputs
    output wire selector_out,
    output wire selector_out_1,
    output wire selector_out_2,
    output wire selector_out_3
);

    wire [3:0] __tmp_1;
    assign __tmp_1 = { 0, self$ };

    wire [3:0] val;
    assign val = 4'd1 << ( 4'd3 - __tmp_1 );

    assign selector_out = val[3];

    assign selector_out_1 = val[2];

    assign selector_out_2 = val[1];

    assign selector_out_3 = val[0];

endmodule

module SSDisplay$encode
(
    // Inputs
    input wire [3:0] n,
    // Outputs
    output wire a,
    output wire b,
    output wire c,
    output wire d,
    output wire e,
    output wire f,
    output wire g
);

    wire [7:0] mux;
    always @(*) begin
        case (n)
            4'b0000 : mux = 8'd126;
            4'b0001 : mux = 8'd48;
            4'b0010 : mux = 8'd109;
            4'b0011 : mux = 8'd121;
            4'b0100 : mux = 8'd51;
            4'b0101 : mux = 8'd91;
            4'b0110 : mux = 8'd95;
            4'b0111 : mux = 8'd112;
            4'b1000 : mux = 8'd127;
            4'b1001 : mux = 8'd123;
            4'b1010 : mux = 8'd119;
            4'b1011 : mux = 8'd31;
            4'b1100 : mux = 8'd78;
            4'b1101 : mux = 8'd61;
            4'b1110 : mux = 8'd79;
            4'b1111 : mux = 8'd71;
            default: mux = 8'd0;
        endcase
    end

    wire [6:0] __tmp_17;
    assign __tmp_17 = mux[0 +: 7];

    assign a = __tmp_17[6];

    assign b = __tmp_17[5];

    assign c = __tmp_17[4];

    assign d = __tmp_17[3];

    assign e = __tmp_17[2];

    assign f = __tmp_17[1];

    assign g = __tmp_17[0];

endmodule

module RoundRobin$mux
(
    // Inputs
    input wire [1:0] self$,
    input wire [3:0] inputs$0,
    input wire [3:0] inputs$1,
    input wire [3:0] inputs$2,
    input wire [3:0] inputs$3,
    // Outputs
    output wire [3:0] mux_out
);

    wire [15:0] __tmp;
    assign __tmp = {
        inputs$3,
        inputs$2,
        inputs$1,
        inputs$0
    };

    always @(*) begin
        case (self$)
            2'b00 : mux_out = __tmp[0 +: 4];
            2'b01 : mux_out = __tmp[4 +: 4];
            2'b10 : mux_out = __tmp[8 +: 4];
            default: mux_out = __tmp[12 +: 4];
        endcase
    end

endmodule

