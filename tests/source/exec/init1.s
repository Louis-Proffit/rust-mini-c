	.text
	.globl main
main:
	movq %rdi, %rdx
	cmpq %rdi, %rdx
	sete %dl
	testq %rdx, %rdx
	jnz L2850
L2848:
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	ret
L2850:
	movq $97, %rdi
	call putchar
	addq $0, %rsp
	jmp L2848
	.data

