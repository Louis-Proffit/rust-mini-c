	.text
	.globl main
main:
	movq %rdx, %rax
	cmpq %rdx, %rax
	sete %al
	testq %rax, %rax
	jnz L2923
L2921:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
L2923:
	movq $97, %rdi
	call putchar
	jmp L2921
	.data

