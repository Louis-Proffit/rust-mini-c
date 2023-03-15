	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq $0, -8(%rbp)
	movq -8(%rbp), %r9
L2043:
	movq -8(%rbp), %rcx
	movq $4, %rax
	cmpq %rax, %rcx
	setle %cl
	testq %rcx, %rcx
	jz L2029
	movq $65, -16(%rbp)
	movq -8(%rbp), %rdi
	call fact
	addq $0, %rsp
	addq %rax, -16(%rbp)
	movq -16(%rbp), %rdi
	call putchar
	addq $0, %rsp
	movq $1, %rsi
	addq %rsi, -8(%rbp)
	movq -8(%rbp), %rcx
	jmp L2043
L2029:
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
fact:
	pushq %rbp
	movq %rsp, %rbp
	addq $-24, %rsp
	movq %rdi, %rax
	movq %rax, -16(%rbp)
	movq $1, -24(%rbp)
	movq -24(%rbp), %r10
	movq -16(%rbp), %r10
	cmpq %r10, %r10
	setle %r10b
	movq %r10, -16(%rbp)
	movq -16(%rbp), %r10
	testq %r10, %r10
	jnz L2021
	movq %rax, -8(%rbp)
	movq %rax, %rdi
	movq $1, %rdx
	subq %rdx, %rdi
	call fact
	addq $0, %rsp
	movq -8(%rbp), %r10
	imulq %rax, %r10
	movq %r10, -8(%rbp)
L2014:
	movq -8(%rbp), %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L2021:
	movq $1, -8(%rbp)
	jmp L2014
	.data

