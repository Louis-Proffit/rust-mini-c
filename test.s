	.text
	.globl main
fact:
	pushq %rbp
	movq %rsp, %rbp
	addq $-24, %rsp
	movq %rdi, %rax
	movq %rax, -16(%rbp)
	movq $1, -24(%rbp)
	movq -24(%rbp), %r10
	movq -16(%rbp), %r11
	cmpq %r10, %r11
	setle %r11b
	movq %r11, -16(%rbp)
	movq -16(%rbp), %r10
	testq %r10, %r10
	jnz L9
	movq %rax, -8(%rbp)
	movq $1, %rcx
	subq %rcx, %rax
	movq %rax, %rdi
	call fact
	movq -8(%rbp), %r11
	imulq %rax, %r11
	movq %r11, -8(%rbp)
L2:
	movq -8(%rbp), %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L9:
	movq $1, -8(%rbp)
	jmp L2
main:
	movq $0, %rax
	ret
	.data

