	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq $65, -16(%rbp)
	movq -16(%rbp), %rax
	movq -16(%rbp), %rdi
	call putchar
	addq $0, %rsp
	movq $1, %rax
	addq %rax, -16(%rbp)
	movq -16(%rbp), %rdx
	movq -16(%rbp), %rdi
	call putchar
	addq $0, %rsp
	movq -16(%rbp), %r10
	movq %r10, -8(%rbp)
	movq $1, %rax
	addq %rax, -8(%rbp)
	movq -8(%rbp), %rsi
	movq -16(%rbp), %rdi
	call putchar
	addq $0, %rsp
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
	.data

