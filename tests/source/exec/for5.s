	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %rsi
L2683:
	movq -8(%rbp), %r9
	testq %r9, %r9
	jz L2672
	movq $65, %rdi
	movq -8(%rbp), %rsi
	addq %rsi, %rdi
	call putchar
	movq $1, %rsi
	subq %rsi, -8(%rbp)
	movq -8(%rbp), %rax
	jmp L2683
L2672:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

