	.text
	.globl main
main:
	movq $42, %rdi
	call print_int
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
print_int:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	movq $10, %rsi
	movq -16(%rbp), %rax
	cqto
	idivq %rsi
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rdx
	movq -8(%rbp), %rsi
	movq $9, %rax
	cmpq %rax, %rsi
	setg %sil
	testq %rsi, %rsi
	jnz L4132
L4130:
	movq $48, %rdi
	movq -8(%rbp), %r9
	movq $10, %rcx
	movq -16(%rbp), %r8
	imulq %r8, %rcx
	subq %rcx, %r9
	addq %r9, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L4132:
	movq -16(%rbp), %rdi
	call print_int
	jmp L4130
	.data

