	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %rcx
L4835:
	movq $1, %r8
	subq %r8, -8(%rbp)
	movq -8(%rbp), %rdi
	testq %rdi, %rdi
	jz L4825
	movq $65, %rdi
	movq -8(%rbp), %rcx
	addq %rcx, %rdi
	call putchar
	addq $0, %rsp
	jmp L4835
L4825:
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

