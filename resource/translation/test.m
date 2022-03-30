           entry      % =====program entry=====
           align      % following instructions align
           addi       r14, r0, topaddr    % stack pointer
           addi       r12, r0, topaddr    % frame pointer
           subi       r14, r14, 4
           subi       r14, r14, 4
           subi       r14, r14, 4
           addi       r1, r0, 2
           sw         -12(r12), r1
           lw         r1, -12(r12)
           sw         -4(r12), r1
           subi       r14, r14, 4
           addi       r4, r0, 3
           sw         -12(r12), r4
           subi       r14, r14, 4
           lw         r1, -4(r12)
           lw         r2, -12(r12)
           mul        r3, r1, r2
           sw         -16(r12), r3
           lw         r1, -16(r12)
           sw         -8(r12), r1
           % load var to print into param reg 
           lw         r1, -4(r12)
           sw         -8(r14), r1
           % load the buffer pointer into param reg 
           addi       r1, r0, buf
           sw         -12(r14), r1
           % call intstr to convert int to str 
           jl         r15, intstr
           % load the result into param reg 
           sw         -8(r14), r13
           jl         r15, putstr
           % load var to print into param reg 
           lw         r1, -8(r12)
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
