	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $16, %rdi
	call malloc
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rsi
	movq $65, %rsi
	movq -8(%rbp), %r8
	movq %rsi, 0(%r8)
	movq -8(%rbp), %r8
	movq 0(%r8), %rdi
	call putchar
	movq $66, %rcx
	movq -8(%rbp), %r8
	movq %rcx, 8(%r8)
	movq -8(%rbp), %rdx
	movq 8(%rdx), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

