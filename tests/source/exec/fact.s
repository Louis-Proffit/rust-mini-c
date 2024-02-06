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
	jnz L2094
	movq %rax, -8(%rbp)
	movq %rax, %rdi
	movq $1, %r9
	subq %r9, %rdi
	call fact
	movq -8(%rbp), %r11
	imulq %rax, %r11
	movq %r11, -8(%rbp)
L2087:
	movq -8(%rbp), %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L2094:
	movq $1, -8(%rbp)
	jmp L2087
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq $0, -8(%rbp)
	movq -8(%rbp), %rdx
L2116:
	movq -8(%rbp), %rax
	movq $4, %r8
	cmpq %r8, %rax
	setle %al
	testq %rax, %rax
	jz L2102
	movq $65, -16(%rbp)
	movq -8(%rbp), %rdi
	call fact
	addq %rax, -16(%rbp)
	movq -16(%rbp), %rdi
	call putchar
	movq $1, %r9
	addq %r9, -8(%rbp)
	movq -8(%rbp), %rcx
	jmp L2116
L2102:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

