	.text
	.globl main
main:
	movq $65, %rdi
	movq $8, %rdx
	addq %rdx, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $65, %rdi
	movq $16, %rsi
	addq %rsi, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	.data

