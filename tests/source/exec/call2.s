	.text
	.globl main
main:
	movq $65, %rdi
	movq $66, %rsi
	movq $67, %rdx
	movq $0, %rcx
	call f
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
f:
	pushq %rbp
	movq %rsp, %rbp
	addq $-32, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq %rdx, -24(%rbp)
	movq %rcx, -32(%rbp)
	movq -8(%rbp), %rax
	testq %rax, %rax
	jnz L1984
	movq $0, %rax
L1976:
	movq %rbp, %rsp
	popq %rbp
	ret
L1984:
	movq -8(%rbp), %rdi
	call putchar
	movq -16(%rbp), %rdi
	movq -24(%rbp), %rsi
	movq -32(%rbp), %rdx
	movq -8(%rbp), %rcx
	call f
	jmp L1976
	.data

