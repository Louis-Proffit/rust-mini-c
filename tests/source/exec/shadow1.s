	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $0, -8(%rbp)
	movq -8(%rbp), %rdi
	movq $1, %r9
	movq %r9, %r8
	movq $1, %rax
	cmpq %rax, %r9
	sete %r9b
	testq %r9, %r9
	jnz L4425
L4423:
	movq -8(%rbp), %r9
	movq $0, %rsi
	cmpq %rsi, %r9
	sete %r9b
	testq %r9, %r9
	jnz L4419
L4417:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L4419:
	movq $98, %rdi
	call putchar
	jmp L4417
L4425:
	movq $97, %rdi
	call putchar
	jmp L4423
	.data

