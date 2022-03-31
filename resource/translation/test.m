           entry      % =====program entry=====
           align      % following instructions align
           addi       r14, r0, topaddr    % stack pointer
           addi       r12, r0, topaddr    % frame pointer
           subi       r14, r14, 20
           subi       r14, r14, 4
           addi       r1, r0, 3
           sw         -24(r12), r1
           lw         r1, -24(r12)
           addi       r2, r0, 20
           subi       r14, r14, 4
           addi       r4, r0, 2
           sw         -28(r12), r4
           lw         r3, -28(r12)
           muli       r3, r3, 4
           sub        r3, r2, r3
           sub        r4, r12, r3
           sw         0(r4), r1
           subi       r14, r12, 20
           subi       r14, r14, 4
           addi       r1, r0, 4
           sw         -24(r12), r1
           lw         r1, -24(r12)
           addi       r3, r0, 20
           subi       r14, r14, 4
           addi       r5, r0, 3
           sw         -28(r12), r5
           lw         r2, -28(r12)
           muli       r2, r2, 4
           sub        r2, r3, r2
           sub        r5, r12, r2
           sw         0(r5), r1
           subi       r14, r12, 20
           subi       r14, r14, 4
           addi       r2, r0, 20
           subi       r14, r14, 4
           addi       r6, r0, 2
           sw         -28(r12), r6
           lw         r3, -28(r12)
           muli       r3, r3, 4
           sub        r3, r2, r3
           sub        r6, r12, r3
           lw         r3, 0(r6)
           sw         -24(r12), r3
           % load var to print into param reg 
           lw         r1, -24(r12)
           sw         -8(r14), r1
           % load the buffer pointer into param reg 
           addi       r1, r0, buf
           sw         -12(r14), r1
           % call intstr to convert int to str 
           jl         r15, intstr
           % load the result into param reg 
           sw         -8(r14), r13
           jl         r15, putstr
           subi       r14, r14, 4
           addi       r6, r0, 20
           subi       r14, r14, 4
           addi       r7, r0, 3
           sw         -36(r12), r7
           lw         r2, -36(r12)
           muli       r2, r2, 4
           sub        r2, r6, r2
           sub        r7, r12, r2
           lw         r2, 0(r7)
           sw         -32(r12), r2
           % load var to print into param reg 
           lw         r1, -32(r12)
           sw         -8(r14), r1
           % load the buffer pointer into param reg 
           addi       r1, r0, buf
           sw         -12(r14), r1
           % call intstr to convert int to str 
           jl         r15, intstr
           % load the result into param reg 
           sw         -8(r14), r13
           jl         r15, putstr
           hlt        % =====end of program====
buf        res        32 % reserve a buffer used by intstr
