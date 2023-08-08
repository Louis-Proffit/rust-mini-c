	.text
	.globl main
main:
	movq $65, %rdi
	movq $66, %rsi
	call f
	movq $66, %rdi
	movq $65, %rsi
	call f
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
f:
	call putchar
	movq $0, %rax
	ret
	.data

