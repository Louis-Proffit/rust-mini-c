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
	movq $1, %rdx
	addq %rdx, -8(%rbp)
	movq -8(%rbp), %rdi
	call putchar
	addq $0, %rsp
	movq -8(%rbp), %rdi
	movq $1, %r9
	addq %r9, %rdi
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

