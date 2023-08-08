	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %r9
L4979:
	movq -8(%rbp), %rcx
	testq %rcx, %rcx
	jz L4968
	movq $65, %rdi
	movq -8(%rbp), %rcx
	addq %rcx, %rdi
	call putchar
	movq $1, %r9
	subq %r9, -8(%rbp)
	movq -8(%rbp), %rdx
	jmp L4979
L4968:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

