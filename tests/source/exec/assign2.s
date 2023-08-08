	.text
	.globl main
main:
	movq $1, %rax
	movq %rax, %rdx
	movq $65, %rdi
	addq %rax, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	.data

