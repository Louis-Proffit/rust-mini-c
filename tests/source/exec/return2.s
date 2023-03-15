	.text
	.globl main
f:
	pushq %rbp
	movq %rsp, %rbp
	addq $-24, %rsp
	movq %rdi, %rax
	movq %rsi, -8(%rbp)
	movq $2, -16(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -24(%rbp)
	movq -24(%rbp), %r10
	movq -16(%rbp), %r10
	imulq %r10, %r10
	movq %r10, -16(%rbp)
	addq -16(%rbp), %rax
	movq %rbp, %rsp
	popq %rbp
	ret
main:
	movq $65, %rdi
	movq $0, %rsi
	call f
	movq %rax, %rdi
	addq $0, %rsp
	call putchar
	addq $0, %rsp
	movq $65, %rdi
	movq $1, %rsi
	call f
	movq %rax, %rdi
	addq $0, %rsp
	call putchar
	addq $0, %rsp
	movq $65, %rdi
	movq $2, %rsi
	call f
	addq $0, %rsp
	movq %rax, %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	ret
	.data

