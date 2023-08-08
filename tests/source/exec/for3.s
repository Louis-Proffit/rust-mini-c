	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %r9
L2606:
	movq -8(%rbp), %rcx
	testq %rcx, %rcx
	jz L2596
	movq $65, %rdi
	movq $1, %r8
	subq %r8, -8(%rbp)
	movq -8(%rbp), %r9
	addq %r9, %rdi
	call putchar
	jmp L2606
L2596:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

