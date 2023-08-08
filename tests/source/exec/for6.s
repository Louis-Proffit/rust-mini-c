	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %rsi
L2727:
	movq $1, %rdi
	subq %rdi, -8(%rbp)
	movq -8(%rbp), %rdi
	movq $1, %rsi
	addq %rsi, %rdi
	testq %rdi, %rdi
	jz L2711
	movq $65, %rdi
	movq -8(%rbp), %rdx
	addq %rdx, %rdi
	call putchar
	movq $1, %r9
	subq %r9, -8(%rbp)
	movq -8(%rbp), %rax
	jmp L2727
L2711:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

