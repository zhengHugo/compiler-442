           entry      % =====program entry=====
           align      % following instructions align
           addi       r14, r0, topaddr    % stack pointer
           addi       r1, r0, topaddr    % frame pointer
           subi       r14, r14, 4
           addi       r2, r0, 2
           sw         -4(r1), r2
           % load integer to be print into param reg 
           addi       r2, r0, 3
           sw         -8(r14), r2
           % load the buffer pointer into param reg 
           addi       r2, r0, buf
           sw         -12(r14), r2
           % call intstr to convert int to str 
           jl         r15, intstr
           % load the result into param reg 
           sw         -8(r14), r13
           jl         r15, putstr
           hlt        % =====end of program====
buf        res        20 % reserve a buffer used by intstr
