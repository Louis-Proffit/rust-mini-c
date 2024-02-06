	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $0, -8(%rbp)
	movq -8(%rbp), %r8
	movq $1, %rcx
	movq %rcx, %rdi
	movq $1, %rdi
	cmpq %rdi, %rcx
	sete %cl
	testq %rcx, %rcx
	jnz L4425
L4423:
	movq -8(%rbp), %rsi
	movq $0, %rax
	cmpq %rax, %rsi
	sete %sil
	testq %rsi, %rsi
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

