	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %rdi
L2495:
	movq -8(%rbp), %r9
	movq $0, %rdi
	cmpq %rdi, %r9
	setg %r9b
	testq %r9, %r9
	jz L2481
	movq $65, %rdi
	movq $1, %r8
	subq %r8, -8(%rbp)
	movq -8(%rbp), %rax
	addq %rax, %rdi
	movq $1, %rdx
	addq %rdx, %rdi
	call putchar
	addq $0, %rsp
	jmp L2495
L2481:
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

