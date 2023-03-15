	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $0, -8(%rbp)
	movq -8(%rbp), %r8
	movq $1, %rdi
	movq %rdi, %r9
	movq $1, %rax
	cmpq %rax, %rdi
	sete %dil
	testq %rdi, %rdi
	jnz L4326
L4324:
	movq -8(%rbp), %rdi
	movq $0, %r9
	cmpq %r9, %rdi
	sete %dil
	testq %rdi, %rdi
	jnz L4320
L4318:
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L4320:
	movq $98, %rdi
	call putchar
	addq $0, %rsp
	jmp L4318
L4326:
	movq $97, %rdi
	call putchar
	addq $0, %rsp
	jmp L4324
	.data

