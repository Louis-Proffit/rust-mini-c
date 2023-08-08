	.text
	.globl main
main:
	movq $65, %rdi
	movq %rdi, %rax
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	.data

