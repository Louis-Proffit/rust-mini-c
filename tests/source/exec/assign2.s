	.text
	.globl main
main:
	movq $1, %rsi
	movq %rsi, %rdx
	movq $65, %rdi
	addq %rsi, %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	ret
	.data

