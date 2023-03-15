	.text
	.globl main
main:
	movq $65, %rdi
	movq %rdi, %rsi
	call putchar
	addq $0, %rsp
	movq $66, %rdi
	movq %rdi, %rdx
	call putchar
	addq $0, %rsp
	movq $67, %rdi
	movq %rdi, %rsi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	ret
	.data

