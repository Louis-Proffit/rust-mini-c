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
	movq $1, %rax
	addq %rax, -8(%rbp)
	movq -8(%rbp), %rdi
	call putchar
	movq -8(%rbp), %rdi
	movq $1, %rdx
	addq %rdx, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

