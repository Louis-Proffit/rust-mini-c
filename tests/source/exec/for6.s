	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %r8
L2654:
	movq $1, %rdi
	subq %rdi, -8(%rbp)
	movq -8(%rbp), %rdi
	movq $1, %r8
	addq %r8, %rdi
	testq %rdi, %rdi
	jz L2638
	movq $65, %rdi
	movq -8(%rbp), %rdx
	addq %rdx, %rdi
	call putchar
	addq $0, %rsp
	movq $1, %rax
	subq %rax, -8(%rbp)
	movq -8(%rbp), %rdx
	jmp L2654
L2638:
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

