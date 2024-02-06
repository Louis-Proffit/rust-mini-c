	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %rax
L2727:
	movq $1, %rax
	subq %rax, -8(%rbp)
	movq -8(%rbp), %r9
	movq $1, %rdi
	addq %rdi, %r9
	testq %r9, %r9
	jz L2711
	movq $65, %rdi
	movq -8(%rbp), %rsi
	addq %rsi, %rdi
	call putchar
	movq $1, %rdi
	subq %rdi, -8(%rbp)
	movq -8(%rbp), %rdx
	jmp L2727
L2711:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

