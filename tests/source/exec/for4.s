	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $10, -8(%rbp)
	movq -8(%rbp), %rsi
L2644:
	movq -8(%rbp), %rax
	testq %rax, %rax
	jz L2634
	movq $65, %rdi
	movq $1, %rax
	subq %rax, -8(%rbp)
	movq -8(%rbp), %rsi
	addq %rsi, %rdi
	call putchar
	jmp L2644
L2634:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

