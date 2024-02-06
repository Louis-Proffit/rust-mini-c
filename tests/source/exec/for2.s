	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %r8
L2568:
	movq -8(%rbp), %r8
	movq $0, %rsi
	cmpq %rsi, %r8
	setg %r8b
	testq %r8, %r8
	jz L2554
	movq $65, %rdi
	movq $1, %rsi
	subq %rsi, -8(%rbp)
	movq -8(%rbp), %rax
	addq %rax, %rdi
	movq $1, %r9
	addq %r9, %rdi
	call putchar
	jmp L2568
L2554:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

