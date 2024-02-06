	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $65, -8(%rbp)
	movq -8(%rbp), %r8
	movq -8(%rbp), %rdi
	call putchar
	movq $1, %r9
	testq %r9, %r9
	jnz L116
	movq $67, %rdi
	movq %rdi, %r8
	call putchar
L108:
	movq -8(%rbp), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L116:
	movq $66, %rdi
	movq %rdi, %rax
	call putchar
	jmp L108
	.data

