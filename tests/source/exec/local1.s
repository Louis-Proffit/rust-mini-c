	.text
	.globl main
main:
	movq $65, %rdi
	movq %rdi, %rax
	call putchar
	movq $66, %rdi
	movq %rdi, %rcx
	call putchar
	movq $67, %rdi
	movq %rdi, %r8
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	.data

