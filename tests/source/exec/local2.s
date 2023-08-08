	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq $65, -8(%rbp)
	movq -8(%rbp), %rcx
	movq -8(%rbp), %rdi
	call putchar
	movq $1, %r8
	addq %r8, -8(%rbp)
	movq -8(%rbp), %rax
	movq -8(%rbp), %rdi
	call putchar
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	movq $1, %rsi
	addq %rsi, -16(%rbp)
	movq -16(%rbp), %rdx
	movq -8(%rbp), %rdi
	call putchar
	movq -16(%rbp), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

