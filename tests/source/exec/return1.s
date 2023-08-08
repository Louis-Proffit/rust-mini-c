	.text
	.globl main
main:
	movq $65, %rdi
	movq $66, %rsi
	movq $67, %rdx
	movq $0, %rcx
	call f
	movq %rax, %rdi
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
	movq $0, %r11
	cmpq %rax, %r11
	sete %al
	testq %rax, %rax
	jnz L4282
	movq -8(%rbp), %rdi
	call putchar
	movq -16(%rbp), %rdi
	movq -24(%rbp), %rsi
	movq -32(%rbp), %rdx
	movq -8(%rbp), %rcx
	call f
L4274:
	movq %rbp, %rsp
	popq %rbp
	ret
L4282:
	movq $10, %rax
	jmp L4274
	.data

