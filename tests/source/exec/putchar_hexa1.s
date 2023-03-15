	.text
	.globl main
main:
	movq $65, %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	ret
	.data

