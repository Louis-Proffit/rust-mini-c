	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %rax
L2568:
	movq -8(%rbp), %rdx
	movq $0, %rsi
	cmpq %rsi, %rdx
	setg %dl
	testq %rdx, %rdx
	jz L2554
	movq $65, %rdi
	movq $1, %rdx
	subq %rdx, -8(%rbp)
	movq -8(%rbp), %rsi
	addq %rsi, %rdi
	movq $1, %rax
	addq %rax, %rdi
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

