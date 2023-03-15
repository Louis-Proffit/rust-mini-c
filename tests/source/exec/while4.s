	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %rdx
L4913:
	movq -8(%rbp), %r8
	testq %r8, %r8
	jz L4902
	movq $1, %rdx
	subq %rdx, -8(%rbp)
	movq -8(%rbp), %rax
	movq $65, %rdi
	movq -8(%rbp), %rcx
	addq %rcx, %rdi
	call putchar
	addq $0, %rsp
	jmp L4913
L4902:
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

