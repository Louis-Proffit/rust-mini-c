	.text
	.globl main
main:
	movq $0, %rdi
	call fact_imp
	movq $1, %rdx
	cmpq %rdx, %rax
	sete %al
	testq %rax, %rax
	jnz L2201
L2199:
	movq $1, %rdi
	call fact_imp
	movq $1, %rsi
	cmpq %rsi, %rax
	sete %al
	testq %rax, %rax
	jnz L2194
L2192:
	movq $5, %rdi
	call fact_imp
	movq $120, %rdi
	cmpq %rdi, %rax
	sete %al
	testq %rax, %rax
	jnz L2187
L2185:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
L2187:
	movq $51, %rdi
	call putchar
	jmp L2185
L2194:
	movq $50, %rdi
	call putchar
	jmp L2192
L2201:
	movq $49, %rdi
	call putchar
	jmp L2199
fact_imp:
	pushq %rbp
	movq %rsp, %rbp
	addq $-64, %rsp
	movq %rdi, -8(%rbp)
	movq $1, %rax
	movq %rax, -64(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -48(%rbp)
	movq $1, -56(%rbp)
	movq -56(%rbp), %r10
	movq -48(%rbp), %r11
	cmpq %r10, %r11
	setg %r11b
	movq %r11, -48(%rbp)
	movq -48(%rbp), %r10
	testq %r10, %r10
	jz L2165
	movq $1, -40(%rbp)
	movq -40(%rbp), %r10
	subq %r10, -8(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -24(%rbp)
	movq $1, -32(%rbp)
	movq -32(%rbp), %r10
	addq %r10, -24(%rbp)
	imulq -24(%rbp), %rax
	movq %rax, -16(%rbp)
	jmp L2179
L2165:
	movq %rbp, %rsp
	popq %rbp
	ret
	.data

