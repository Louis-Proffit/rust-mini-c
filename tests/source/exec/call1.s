	.text
	.globl main
f:
	call putchar
	addq $0, %rsp
	movq $0, %rax
	ret
main:
	movq $65, %rdi
	movq $66, %rsi
	call f
	addq $0, %rsp
	movq $66, %rdi
	movq $65, %rsi
	call f
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	ret
	.data

