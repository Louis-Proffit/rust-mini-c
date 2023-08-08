	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $65, -8(%rbp)
	movq -8(%rbp), %rsi
	movq -8(%rbp), %rdi
	call putchar
	movq -8(%rbp), %rsi
	testq %rsi, %rsi
	jnz L2874
L2872:
	movq -8(%rbp), %rdi
	call putchar
	movq -8(%rbp), %rax
	testq %rax, %rax
	jz L2865
	movq $0, %rax
	testq %rax, %rax
	jz L2865
	movq $1, %rax
L2865:
	testq %rax, %rax
	jnz L2864
L2862:
	movq -8(%rbp), %rdi
	call putchar
	movq -8(%rbp), %rdi
	testq %rdi, %rdi
	jz L2855
	movq $1, %rdi
	testq %rdi, %rdi
	jz L2855
	movq $1, %rdi
L2855:
	testq %rdi, %rdi
	jnz L2854
L2852:
	movq -8(%rbp), %rdi
	call putchar
	movq -8(%rbp), %r8
	testq %r8, %r8
	jnz L2846
	movq $0, %r8
	testq %r8, %r8
	jz L2845
L2846:
	movq $1, %r8
L2845:
	testq %r8, %r8
	jnz L2844
L2842:
	movq -8(%rbp), %rdi
	call putchar
	movq -8(%rbp), %rdi
	testq %rdi, %rdi
	jnz L2836
	movq $1, %rdi
	testq %rdi, %rdi
	jz L2835
L2836:
	movq $1, %rdi
L2835:
	testq %rdi, %rdi
	jnz L2834
L2832:
	movq -8(%rbp), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L2834:
	movq $70, -8(%rbp)
	movq -8(%rbp), %rdi
	jmp L2832
	jmp L2835
	jmp L2836
L2844:
	movq $69, -8(%rbp)
	movq -8(%rbp), %r9
	jmp L2842
	jmp L2845
	jmp L2846
L2854:
	movq $68, -8(%rbp)
	movq -8(%rbp), %rdx
	jmp L2852
	jmp L2855
	jmp L2855
L2864:
	movq $67, -8(%rbp)
	movq -8(%rbp), %r8
	jmp L2862
	jmp L2865
	jmp L2865
L2874:
	movq $66, -8(%rbp)
	movq -8(%rbp), %r9
	jmp L2872
	.data

