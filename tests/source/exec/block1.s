	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $65, -8(%rbp)
	movq -8(%rbp), %rdx
	movq -8(%rbp), %rdi
	call putchar
	movq $1, %rax
	testq %rax, %rax
	jnz L71
	movq $67, %rdi
	movq %rdi, %r9
	call putchar
L18:
	movq -8(%rbp), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L71:
	movq $66, %rdi
	movq %rdi, %rax
	call putchar
	jmp L18
	.data

