	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $65, -8(%rbp)
	movq -8(%rbp), %r8
	movq -8(%rbp), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rax
	testq %rax, %rax
	jnz L2801
L2799:
	movq -8(%rbp), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rcx
	testq %rcx, %rcx
	jz L2792
	movq $0, %rcx
	testq %rcx, %rcx
	jz L2792
	movq $1, %rcx
L2792:
	testq %rcx, %rcx
L2792:
	jnz L2791
L2789:
	movq -8(%rbp), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rcx
	testq %rcx, %rcx
	jz L2782
	movq $1, %rcx
	testq %rcx, %rcx
	jz L2782
	movq $1, %rcx
L2782:
	testq %rcx, %rcx
L2782:
	jnz L2781
L2779:
	movq -8(%rbp), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rax
	testq %rax, %rax
	jnz L2773
	movq $0, %rax
	testq %rax, %rax
	jz L2772
L2773:
	movq $1, %rax
L2772:
	testq %rax, %rax
L2772:
	jnz L2771
L2769:
	movq -8(%rbp), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %r8
	testq %r8, %r8
	jnz L2763
	movq $1, %r8
	testq %r8, %r8
	jz L2762
L2763:
	movq $1, %r8
L2762:
	testq %r8, %r8
L2762:
	jnz L2761
L2759:
	movq -8(%rbp), %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L2761:
	movq $70, -8(%rbp)
	movq -8(%rbp), %r9
	jmp L2759
	jmp L2762
	jmp L2763
L2771:
	movq $69, -8(%rbp)
	movq -8(%rbp), %rdx
	jmp L2769
	jmp L2772
	jmp L2773
L2781:
	movq $68, -8(%rbp)
	movq -8(%rbp), %rdi
	jmp L2779
	jmp L2782
	jmp L2782
L2791:
	movq $67, -8(%rbp)
	movq -8(%rbp), %rsi
	jmp L2789
	jmp L2792
	jmp L2792
L2801:
	movq $66, -8(%rbp)
	movq -8(%rbp), %rdx
	jmp L2799
	.data

