	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %rcx
L2533:
	movq -8(%rbp), %rdi
	testq %rdi, %rdi
	jz L2523
	movq $65, %rdi
	movq $1, %rcx
	subq %rcx, -8(%rbp)
	movq -8(%rbp), %rcx
	addq %rcx, %rdi
	call putchar
	addq $0, %rsp
	jmp L2533
L2523:
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

