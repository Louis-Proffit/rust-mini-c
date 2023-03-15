	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %rax
L2610:
	movq -8(%rbp), %rdi
	testq %rdi, %rdi
	jz L2599
	movq $65, %rdi
	movq -8(%rbp), %r8
	addq %r8, %rdi
	call putchar
	addq $0, %rsp
	movq $1, %r8
	subq %r8, -8(%rbp)
	movq -8(%rbp), %rax
	jmp L2610
L2599:
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

