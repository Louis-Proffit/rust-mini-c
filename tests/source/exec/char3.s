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
	addq $0, %rsp
	movq $1, %r9
	addq %r9, -8(%rbp)
	movq -8(%rbp), %r9
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

