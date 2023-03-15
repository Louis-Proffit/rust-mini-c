	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %r9
L2571:
	movq -8(%rbp), %rdx
	testq %rdx, %rdx
	jz L2561
	movq $65, %rdi
	movq $1, %rcx
	subq %rcx, -8(%rbp)
	movq -8(%rbp), %rdx
	addq %rdx, %rdi
	call putchar
	addq $0, %rsp
	jmp L2571
L2561:
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

