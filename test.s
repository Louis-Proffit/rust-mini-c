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
	cmpq %r10, -16(%rbp)
	setle -16(%rbp)
	movq -16(%rbp), %r10
	testq %r10, %r10
	jnz L10
	movq %rax, -8(%rbp)
	movq %rax, %rdi
	movq $1, %rsi
	subq %rsi, %rdi
	call fact
	addq $0, %rsp
	movq -8(%rbp), %r10
	imulq %rax, %r10
	movq %r10, -8(%rbp)
L3:
	movq -8(%rbp), %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L10:
	movq $1, -8(%rbp)
	jmp L3
main:
	movq $42, %rdi
	call fact
	addq $0, %rsp
	ret
	.data

